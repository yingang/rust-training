#[allow(dead_code)]
enum Gender {
    Unspecified= 0,
    Male,
    Female,
}

#[allow(dead_code)]
struct User {
    name: String,
    age: u8,
    gender: Gender,
}

#[allow(dead_code)]
enum ConnectionState {
    Init,
    SyncReceived(HalfOpen),
    SyncAckSent(HalfOpen),
    AckReceived(FullSession),
}

#[allow(dead_code)]
struct HalfOpen {}
#[allow(dead_code)]
struct FullSession {}
