use priv_prelude::*;

/// Adds latency to an IP connection
pub struct IpLatency {
    //inner: Latency<IpPacket>,
}

impl IpLatency {
    /// Connect the two given plugs with latency added to the connection.
    ///
    /// `min_latency` is the baseline for the amount of delay added to packets travelling along
    /// this connection. `mean_additional_latency` controls the amount of random, additional
    /// latency added to any given packet. A non-zero `mean_additional_latency` can cause packets
    /// to be re-ordered.
    pub fn spawn(
        handle: &NetworkHandle,
        min_latency: Duration,
        mean_additional_latency: Duration,
        plug_a: IpPlug,
        plug_b: IpPlug,
    ) {

        Latency::spawn(
            handle,
            min_latency,
            mean_additional_latency,
            plug_a.into(),
            plug_b.into(),
        )
    }
}

