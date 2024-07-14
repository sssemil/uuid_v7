use std::sync::atomic::{AtomicU64, Ordering};

use chrono::{DateTime, Utc};
use rand::Rng;
use uuid::Uuid;

static LAST_TIMESTAMP: AtomicU64 = AtomicU64::new(0);

pub fn gen_uuid_v7() -> Uuid {
    let now: DateTime<Utc> = Utc::now();
    let mut timestamp = now.timestamp_millis() as u64;

    loop {
        let last_timestamp = LAST_TIMESTAMP.load(Ordering::SeqCst);

        if timestamp <= last_timestamp {
            timestamp = last_timestamp + 1;
        }

        if LAST_TIMESTAMP
            .compare_exchange(
                last_timestamp,
                timestamp,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok()
        {
            break;
        }
    }

    // This bit was probably copied from some other repo, but I don't remember where I got it from
    // anymore
    let mut bytes = [0u8; 16];

    // Fill the timestamp in the first 48 bits (6 bytes)
    let timestamp_bytes = timestamp.to_be_bytes();
    bytes[0..6].copy_from_slice(&timestamp_bytes[2..8]);

    // Fill the remaining bytes with random data
    let mut rng = rand::thread_rng();
    bytes[6..].copy_from_slice(&rng.gen::<[u8; 10]>());

    // Set the UUID version (0111 for v7) and variant (10)
    bytes[6] &= 0x0F; // Clear the top 4 bits
    bytes[6] |= 0x70; // Set the version (0111)
    bytes[8] &= 0xBF; // Set the top 2 bits to 10
    bytes[8] |= 0x80;

    Uuid::from_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use uuid::Uuid;

    use super::gen_uuid_v7;

    #[test]
    fn test_uuid_validity() {
        // Generate a UUID
        let uuid = gen_uuid_v7();

        // Check if it's a valid UUID
        assert!(Uuid::parse_str(&uuid.to_string()).is_ok());
    }

    #[test]
    fn test_uuid_uniqueness() {
        let mut set = HashSet::new();
        for _ in 0..1000 {
            // Generate a UUID
            let uuid = gen_uuid_v7();

            // Insert into the set and ensure it's unique
            assert!(set.insert(uuid));
        }
    }

    #[test]
    fn test_uuid_version() {
        // Generate a UUID
        let uuid = gen_uuid_v7();

        // Check if it's version 7
        assert_eq!(uuid.get_version_num(), 7);
    }

    #[test]
    fn test_uuid_increasing_many() {
        let uuids = (0..1000).map(|_| gen_uuid_v7()).collect::<Vec<_>>();
        for i in 1..uuids.len() {
            assert!(
                uuids[i] > uuids[i - 1],
                "UUIDs are not monotonically increasing"
            );
        }
    }
}
