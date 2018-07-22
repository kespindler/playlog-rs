use record::Record;


pub trait Handler {
    fn write(&self, record: &Record) -> ();
}