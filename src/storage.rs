#![allow(dead_code)]

use std::io::Write;

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;
const ROW_SIZE: usize = 8 + COLUMN_USERNAME_SIZE + COLUMN_EMAIL_SIZE;
const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

// NOTE(alvaro): At this point the row has the following format
//
// id: INTEGER
// username: VARCHAR(32)
// email: VARCHAR(255)

#[derive(Debug, PartialEq, Eq)]
pub struct Row {
    id: u64,
    username: String,
    email: String,
}

impl Row {
    pub fn new(id: u64, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
        }
    }

    /// Serialize a row
    ///
    /// The format contains the size of each field and the field, one after
    /// the other
    pub fn serialize(&self, mut dest: &mut [u8]) -> anyhow::Result<usize> {
        let mut written = 0;

        let id_bytes = self.id.to_le_bytes();
        let id_len = std::mem::size_of::<u64>() as u64;
        dest.write_all(id_len.to_le_bytes().as_ref())?;
        dest.write_all(id_bytes.as_ref())?;
        written += id_len as usize + id_bytes.len();

        let mut username_bytes = [0; COLUMN_USERNAME_SIZE];
        username_bytes[..self.username.as_bytes().len()].copy_from_slice(self.username.as_bytes());
        let username_len = username_bytes.len() as u64;

        dest.write_all(username_len.to_le_bytes().as_ref())?;
        dest.write_all(&username_bytes)?;
        written += id_len as usize + username_bytes.len();

        let mut email_bytes = [0; COLUMN_EMAIL_SIZE];
        email_bytes[..self.email.as_bytes().len()].copy_from_slice(self.email.as_bytes());
        let email_len = email_bytes.len() as u64;
        dest.write_all(email_len.to_le_bytes().as_ref())?;
        dest.write_all(&email_bytes)?;
        written += id_len as usize + email_bytes.len();

        Ok(written)
    }

    /// Deserialize a row from `data`
    ///
    /// Returns a `Row` and a slice to the rest of the data
    pub fn deserialize(data: &[u8]) -> anyhow::Result<(Self, &[u8])> {
        let len_size = std::mem::size_of::<u64>();

        // Read the id
        let id_len = u64::from_le_bytes(data[..len_size].try_into()?) as usize;
        let data = &data[len_size..];

        let id = u64::from_le_bytes(data[..id_len].try_into()?);
        let data = &data[id_len..];

        // Read the username
        let username_len = u64::from_le_bytes(data[..len_size].try_into()?) as usize;
        let data = &data[len_size..];
        let username = read_null_terminated_str(&data[..username_len])?;
        let data = &data[username_len..];

        // Read the email
        let email_len = u64::from_le_bytes(data[..len_size].try_into()?) as usize;
        let data = &data[len_size..];
        let email = read_null_terminated_str(&data[..email_len])?;
        let data = &data[email_len..];

        let row = Row::new(id, username.to_string(), email.to_string());

        Ok((row, data))
    }
}

#[derive(Debug)]
struct Table {
    num_rows: usize,
    pages: Vec<Option<Page>>,
}

impl Table {
    /// Return a mutable reference to the data inside a Page
    pub fn row_slot_mut(&mut self, row_num: usize) -> &mut [u8] {
        let page_num = row_num / ROWS_PER_PAGE;
        self.check_slot(page_num);
        let page = self.pages[page_num].as_mut().unwrap();

        // Find the row offset inside the page data
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        &mut page.data[byte_offset..byte_offset + ROW_SIZE]
    }

    /// Check the Vec of pages to see if the page already exists, or allocate
    /// if it does not
    fn check_slot(&mut self, page_num: usize) {
        assert!(page_num < self.pages.len());

        // Allocate a page if it does not exist already
        if self.pages.get(page_num).is_none() {
            self.pages[page_num] = Some(Page::empty());
        }
    }
}

#[derive(Debug)]
struct Page {
    data: Vec<u8>,
}

impl Page {
    pub fn empty() -> Self {
        Page {
            data: vec![0; PAGE_SIZE],
        }
    }
}

fn read_null_terminated_str(data: &[u8]) -> anyhow::Result<&str> {
    let null_pos = data.iter().position(|c| *c == 0).unwrap_or(data.len());
    Ok(std::str::from_utf8(&data[..null_pos])?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_round_trip() {
        let row = Row::new(1, "foo".to_string(), "foo@example.org".to_string());

        // Serialize
        let mut buffer: Vec<u8> = vec![0; 1024];
        let written = row.serialize(&mut buffer).unwrap();
        // id_len (u64) + id (u64) + username_len (u64) + username + email_len (u64) + email
        let expected_written = std::mem::size_of::<u64>() as u64 * 4
            + COLUMN_USERNAME_SIZE as u64
            + COLUMN_EMAIL_SIZE as u64;
        assert_eq!(written as u64, expected_written);

        let (new_row, rest) = Row::deserialize(&buffer).unwrap();
        assert_eq!(row, new_row);
        // The rest of the buffer is all zeros
        assert!(rest.iter().all(|e| *e == 0));
    }
}
