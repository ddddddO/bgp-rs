/// https://datatracker.ietf.org/doc/html/rfc4271#section-8.1
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Event {
  ManualStart,

  // 正常系のみの実装を想定しているため、TcpCrAckedも兼ねている、そう
  TcpConnectionConfirmed,
}
