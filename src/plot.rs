use std::{collections::BTreeMap, error::Error, io, ops, time::Duration};

use plotters::prelude::*;

use crate::record::Record;

const FONT: &str = "Fira Code";

#[derive(Debug, Default)]
#[repr(transparent)]
pub(crate) struct Groups(BTreeMap<String, Vec<Record>>);

impl ops::Deref for Groups {
    type Target = BTreeMap<String, Vec<Record>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Groups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<BTreeMap<String, Vec<Record>>> for Groups {
    fn from(groups: BTreeMap<String, Vec<Record>>) -> Self {
        Groups(groups)
    }
}

const COLORS: &[RGBColor] = &[
    RGBColor(251, 26, 26),
    RGBColor(248, 111, 28),
    RGBColor(248, 193, 26),
    RGBColor(223, 252, 24),
    RGBColor(131, 242, 20),
    RGBColor(63, 250, 37),
    RGBColor(30, 243, 83),
    RGBColor(49, 251, 175),
    RGBColor(46, 251, 251),
    RGBColor(11, 162, 252),
    RGBColor(48, 98, 247),
    RGBColor(46, 16, 253),
    RGBColor(143, 35, 251),
    RGBColor(221, 52, 246),
    RGBColor(246, 58, 199),
    RGBColor(250, 9, 99),
];

impl Groups {
    pub(crate) fn init() -> Self {
        let mut groups = Self::default();

        csv::Reader::from_reader(io::stdin())
            .deserialize()
            .map(|result| result.expect("invalid record"))
            .for_each(|record: Record| {
                let group = groups.entry(record.name.clone()).or_insert_with(Vec::new);
                group.push(record);
            });

        groups
    }

    pub(crate) fn plot_throughput(
        self,
        dir: &str,
        name: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, Box<dyn Error>> {
        let path = format!("{}/{}.throughput.svg", dir, name);

        let root = SVGBackend::new(&path, (width, height)).into_drawing_area();

        root.fill(&WHITE)?;

        let (x_max, y_max) = self
            .values()
            .flatten()
            .map(|record| (record.threads, record.throughput))
            .fold((0, 0f64), |res, cur| (res.0.max(cur.0), res.1.max(cur.1)));

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption(
                &format!("{}: Throughput @ {}", name, env!("CARGO_PKG_VERSION")),
                (FONT, 20),
            )
            .set_label_area_size(LabelAreaPosition::Left, 70)
            .set_label_area_size(LabelAreaPosition::Right, 70)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(1..x_max, 0.0..y_max)?;

        chart
            .configure_mesh()
            .disable_y_mesh()
            .x_label_formatter(&|v| format!("{}", v))
            .y_label_formatter(&|v| format!("{:.0} Mop/s", v / 1_000_000.))
            .x_labels(20)
            .y_desc("Throughput")
            .x_desc("Threads")
            .draw()?;

        let colors = COLORS.iter().cycle();

        for (records, color) in self.values().zip(colors) {
            chart
                .draw_series(LineSeries::new(
                    records
                        .iter()
                        .map(|record| (record.threads, record.throughput)),
                    color,
                ))?
                .label(&records[0].name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }

        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .label_font((FONT, 13))
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        Ok(self)
    }

    pub(crate) fn plot_latency(
        self,
        dir: &str,
        name: &str,
        width: u32,
        height: u32,
        latency_limit_ns: u64,
    ) -> Result<Self, Box<dyn Error>> {
        let path = format!("{}/{}.latency.svg", dir, name);

        let root = SVGBackend::new(&path, (width, height)).into_drawing_area();

        root.fill(&WHITE)?;

        let (x_max, y_max) = self
            .values()
            .flatten()
            .map(|record| (record.threads, record.latency))
            .fold((0, Duration::from_secs(0)), |res, cur| {
                (res.0.max(cur.0), res.1.max(cur.1))
            });

        let y_max = latency_limit_ns.min(y_max.as_nanos() as u64);

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption(
                &format!("{}: Latency @ {}", name, env!("CARGO_PKG_VERSION")),
                (FONT, 20),
            )
            .set_label_area_size(LabelAreaPosition::Left, 70)
            .set_label_area_size(LabelAreaPosition::Right, 70)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(1..x_max, 0..y_max)?;

        chart
            .configure_mesh()
            .disable_y_mesh()
            .x_label_formatter(&|v| format!("{}", v))
            .y_label_formatter(&|v| format!("{:.0} ns", v))
            .x_labels(20)
            .y_labels(20)
            .y_desc("Latency")
            .x_desc("Threads")
            .draw()?;

        let colors = COLORS.iter().cycle();

        for (records, color) in self.values().zip(colors) {
            chart
                .draw_series(LineSeries::new(
                    records
                        .iter()
                        .map(|record| (record.threads, record.latency.as_nanos() as u64)),
                    color,
                ))?
                .label(&records[0].name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }

        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .label_font((FONT, 13))
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        Ok(self)
    }
}
