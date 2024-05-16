use core::slice::Iter;
use plotly::color::NamedColor;
use plotly::common::{Marker, Title};
use plotly::layout::{Axis, Layout};
use plotly::{Histogram, Plot, Scatter};
use yew::prelude::*;

pub trait MGFLike: PartialEq {
    type MZ<'a>: IntoIterator<Item = &'a f64> + 'a
    where
        Self: 'a;

    type Intensity<'b>: IntoIterator<Item = &'b f64> + 'b
    where
        Self: 'b;
    fn iter_mz2(&self) -> Self::MZ<'_>;
    fn iter_mz2_intensity(&self) -> Self::Intensity<'_>;

    fn get_ms1_mass(&self) -> f64;
    fn get_ms1_retention_time(&self) -> f64;
}

#[derive(Clone, Properties, PartialEq)]
pub struct PlotComponentProps<S: MGFLike> {
    pub data: S,
}

#[derive(Clone, PartialEq)]
struct ExampleMGF {
    ms1_mass: f64,
    ms1_retention_time: f64,
    ms2_mass: Vec<f64>,
    ms2_intensity: Vec<f64>,
}

impl MGFLike for ExampleMGF {
    type MZ<'a> = Iter<'a, f64>;
    type Intensity<'b> = Iter<'b, f64>;
    fn iter_mz2(&self) -> Self::MZ<'_> {
        self.ms2_mass.iter()
    }
    fn iter_mz2_intensity(&self) -> Self::Intensity<'_> {
        self.ms2_intensity.iter()
    }

    fn get_ms1_mass(&self) -> f64 {
        self.ms1_mass
    }

    fn get_ms1_retention_time(&self) -> f64 {
        self.ms1_retention_time
    }
}

impl ExampleMGF {
    pub fn example1() -> Self {
        ExampleMGF {
            ms1_mass: 520.3401184082031,
            ms1_retention_time: 450.62851800000004,
            ms2_mass: vec![
                51.556331634521484,
                55.05524826049805,
                57.815757751464844,
                58.06614685058594,
                60.08164978027344,
                60.508689880371094,
                67.05503845214844,
                69.07072448730469,
                71.07373809814453,
                79.05477142333984,
                81.070556640625,
                82.82237243652344,
                83.08595275878906,
                86.09703826904297,
                87.95624542236328,
                93.07064819335938,
                94.95048522949219,
                95.0859375,
                96.22694396972656,
                96.87842559814453,
                98.61199951171875,
                98.98477172851562,
                104.09294128417969,
                104.10742950439453,
                104.11273956298828,
                107.08605194091797,
                109.1014633178711,
                123.11724090576172,
                124.99996948242188,
                125.00931549072266,
                137.55953979492188,
                163.01519775390625,
                184.03955078125,
                184.07337951660156,
                236.634033203125,
                258.1107177734375,
                337.2723083496094,
                434.93133544921875,
                438.89288330078125,
                459.9193420410156,
                502.32794189453125,
                520.3394165039062,
            ],
            ms2_intensity: vec![
                1.8E6, 1.1E6, 4.0E6, 2.5E6, 4.2E7, 1.1E6, 5.5E6, 2.3E6, 9.4E6, 2.5E6, 5.1E6, 1.3E6,
                1.4E6, 1.2E8, 1.2E6, 1.9E6, 1.2E6, 5.8E6, 1.1E6, 4.1E6, 1.1E6, 3.3E6, 2.0E6, 3.3E8,
                7.3E6, 1.7E6, 2.1E6, 1.3E6, 5.9E7, 1.7E6, 1.3E6, 3.1E6, 1.8E6, 3.3E8, 1.2E6, 2.4E6,
                2.6E6, 1.3E6, 1.3E6, 1.4E6, 1.3E7, 7.3E7,
            ],
        }
    }
}

#[function_component(PlotComponent)]
pub fn plot_component<S: MGFLike>(props: &PlotComponentProps<S>) -> Html {
    let p: yew_hooks::prelude::UseAsyncHandle<(), ()> = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let mut plot = Plot::new();

        let trace = Scatter::new(vec![0.0, 0.0], vec![0.0, 0.0])
            .show_legend(false)
            .line(
                plotly::common::Line::new()
                    .color(NamedColor::Black)
                    .width(0.1),
            )
            .marker(Marker::new().size(1));
        plot.add_trace(trace);

        for (mz, intensity) in props
            .data
            .iter_mz2()
            .into_iter()
            .zip(props.data.iter_mz2_intensity().into_iter())
        {
            let trace = Scatter::new(vec![*mz, *mz], vec![0.0, *intensity])
                .line(
                    plotly::common::Line::new()
                        .color(NamedColor::Black)
                        .width(1.7),
                )
                .show_legend(false)
                .marker(Marker::new().size(1))
                .name(format!("{}", mz));
            plot.add_trace(trace);
        }

        let layout = plotly::Layout::new()
            .title(plotly::common::Title::new(&format!(
                "MS2 Spectrum of mass {:.4} m/z at RT {:.4} seconds",
                props.data.get_ms1_mass(),
                props.data.get_ms1_retention_time()
            )))
            .x_axis(Axis::new().title(Title::new("m/z")))
            .y_axis(
                Axis::new()
                    .title(Title::new("Intensity"))
                    .exponent_format(plotly::common::ExponentFormat::CapitalE),
            );
        plot.set_layout(layout);

        async move {
            plotly::bindings::new_plot(id, &plot).await;
            Ok(())
        }
    });

    use_effect_with_deps(
        move |_| {
            p.run();
            || ()
        },
        (),
    );

    html! {
        <div id="plot-div"></div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let mgf = ExampleMGF::example1();
    html! {
        <PlotComponent<ExampleMGF> data={mgf} />
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
