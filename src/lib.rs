pub struct CompletionQueue {
    _ring: std::sync::Arc<iou::IoUring>,
    cq: iou::CompletionQueue<'static>,
}

pub struct SubmissionQueue {
    _ring: std::sync::Arc<iou::IoUring>,
    sq: iou::SubmissionQueue<'static>,
}

pub struct Registrar {
    _ring: std::sync::Arc<iou::IoUring>,
    registrar: iou::Registrar<'static>,
}

pub fn new(events: u32) -> std::io::Result<(CompletionQueue, SubmissionQueue, Registrar)> {
    let mut ring = iou::IoUring::new(events)?;
    let cq: iou::CompletionQueue<'_> = ring.cq();
    let cq: iou::CompletionQueue<'static> = unsafe { core::mem::transmute(cq) };
    let sq: iou::SubmissionQueue<'_> = ring.sq();
    let sq: iou::SubmissionQueue<'static> = unsafe { core::mem::transmute(sq) };
    let registrar: iou::Registrar<'_> = ring.registrar();
    let registrar: iou::Registrar<'static> = unsafe { core::mem::transmute(registrar) };
    let ring = std::sync::Arc::new(ring);
    let cq = CompletionQueue {
        _ring: ring.clone(),
        cq,
    };
    let sq = SubmissionQueue {
        _ring: ring.clone(),
        sq,
    };
    let r = Registrar {
        _ring: ring.clone(),
        registrar,
    };
    Ok((cq, sq, r))
}

impl SubmissionQueue {
    pub fn prepare_sqe(&mut self) -> Option<iou::SQE<'_>> {
        self.sq.prepare_sqe()
    }

    pub fn submit(&mut self) -> std::io::Result<u32> {
        self.sq.submit()
    }
}

impl CompletionQueue {
    pub fn wait_for_cqe(&mut self) -> std::io::Result<iou::CQE> {
        self.cq.wait_for_cqe()
    }
}
