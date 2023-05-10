// use std::error::Error;
// use std::fmt::{Debug, Display, Formatter};
//
// use uuid::Uuid;
//
// #[derive(Debug)]
// pub enum Err {
//     NoOpenedChannel { customs_id: Uuid },
//     ChannelWasClosed { customs_id: Uuid },
//     FailedToReceive,
//     FailedToSend,
// }
//
// impl Display for Err {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Err::NoOpenedChannel { customs_id } => write!(
//                 f,
//                 "Customs (id={customs_id}) has no opened channel with Processor"
//             ),
//             Err::ChannelWasClosed { customs_id } => write!(
//                 f,
//                 "Customs (id={customs_id}) has it's channel with Processor closed"
//             ),
//             Err::FailedToReceive => todo!(),
//             Err::FailedToSend => todo!(),
//         }
//     }
// }
//
// impl Error for Err {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }
// }
