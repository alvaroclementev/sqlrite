use std::io::Write;

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

        let username_bytes = self.username.as_bytes();
        let username_len = username_bytes.len() as u64;
        dest.write_all(username_len.to_le_bytes().as_ref())?;
        dest.write_all(username_bytes)?;
        written += id_len as usize + username_bytes.len();

        let email_bytes = self.email.as_bytes();
        let email_len = email_bytes.len() as u64;
        dest.write_all(email_len.to_le_bytes().as_ref())?;
        dest.write_all(email_bytes)?;
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
        let username = std::str::from_utf8(&data[..username_len])?;
        let data = &data[username_len..];

        // Read the email
        let email_len = u64::from_le_bytes(data[..len_size].try_into()?) as usize;
        let data = &data[len_size..];
        let email = std::str::from_utf8(&data[..email_len])?;
        let data = &data[email_len..];

        let row = Row::new(id, username.to_string(), email.to_string());

        Ok((row, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let row = Row::new(1, "foo".to_string(), "foo@example.org".to_string());

        // Serialize
        let mut buffer: Vec<u8> = vec![0; 1024];
        let written = row.serialize(&mut buffer).unwrap();
        // id_len (u64) + id (u64) + username_len (u64) + username + email_len (u64) + email
        let expected_written = std::mem::size_of::<u64>() as u64 * 4
            + row.username.as_bytes().len() as u64
            + row.email.as_bytes().len() as u64;
        assert_eq!(written as u64, expected_written);

        let (new_row, rest) = Row::deserialize(&buffer).unwrap();
        assert_eq!(row, new_row);
        // The rest of the buffer is all zeros
        assert!(rest.iter().all(|e| *e == 0));
    }
}
