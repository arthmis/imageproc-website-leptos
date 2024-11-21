use leptos::{create_rw_signal, leptos_dom::Text, IntoView, RwSignal, SignalSet, View};

pub struct AlgorithmInputState {
    gamma: RwSignal<f64>,
    invert: RwSignal<bool>,
    box_blur_amount: RwSignal<u32>,
    sobel_edge_detector_threshold: RwSignal<u32>,
}

impl Default for AlgorithmInputState {
    fn default() -> Self {
        Self {
            gamma: create_rw_signal(1.),
            invert: create_rw_signal(false),
            box_blur_amount: create_rw_signal(1u32),
            sobel_edge_detector_threshold: create_rw_signal(128u32),
        }
    }
}

impl AlgorithmInputState {
    pub fn gamma(&self) -> RwSignal<f64> {
        self.gamma
    }
    pub fn invert(&self) -> RwSignal<bool> {
        self.invert
    }
    pub fn box_blur_amount(&self) -> RwSignal<u32> {
        self.box_blur_amount
    }
    pub fn sobel_edge_detector_threshold(&self) -> RwSignal<u32> {
        self.sobel_edge_detector_threshold
    }

    pub fn reset(&self) {
        self.invert.set(false);
        self.box_blur_amount.set(1);
        self.gamma.set(1.);
        self.sobel_edge_detector_threshold.set(128)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Algorithm {
    Gamma,
    Invert,
    BoxBlur,
    SobelEdgeDetector,
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Algorithm::Gamma => "gamma",
            Algorithm::Invert => "invert",
            Algorithm::BoxBlur => "box blur",
            Algorithm::SobelEdgeDetector => "sobel edge detector",
        };
        write!(f, "{}", text)
    }
}

impl IntoView for Algorithm {
    fn into_view(self) -> View {
        View::Text(Text::new(self.to_string().into()))
    }
}
