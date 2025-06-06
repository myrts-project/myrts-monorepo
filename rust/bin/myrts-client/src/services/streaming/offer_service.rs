/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use crate::states::ClientState;
use proto::{
    app::{Data, MsgData, Stream},
    error::Result,
};
use types::proto::Offer;

#[proto::service("offer")]
async fn offer(stream: Stream, data: MsgData<Offer>, state: Data<ClientState>) -> Result<()> {
    let data = data.into_inner();
    tokio::spawn(async move {
        state.start_streaming(stream, data.offer).await;
    });
    Ok(())
}
