use rust_extensions::date_time::DateTimeAsMicroseconds;

pub trait MyNoSqlEntity {
    fn get_partition_key(&self) -> &str;
    fn get_row_key(&self) -> &str;
    fn get_time_stamp(&self) -> DateTimeAsMicroseconds;
}
