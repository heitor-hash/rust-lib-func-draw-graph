// Mod para função do segundo grau
use plotters::prelude::*;
#[allow(unused)]

fn line1_style(res_mult: f64) -> ShapeStyle {
    ShapeStyle {
        color: RGBAColor(255, 0, 0, 0.8),
        filled: false,
        stroke_width: (2f64 * res_mult) as u32,
    }
}

fn f3_deg_to_range(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    step: f64,
    x_min: i32,
    x_max: i32,
) -> impl Iterator<Item = (f64, f64)> {
    let x_min: f64 = x_min as f64 / step;
    let x_max: f64 = x_max as f64 / step;

    let range = (x_min as i32..=x_max as i32).map(move |i| {
        let x: f64 = i as f64 * step;
        (x, a * x.powf(3.0) + b * x.powf(2.0) + c * x + d)
    });
    range
}

fn f2_deg_to_range(
    a: f64,
    b: f64,
    c: f64,
    step: f64,
    x_min: i32,
    x_max: i32,
) -> impl Iterator<Item = (f64, f64)> {
    let x_min: f64 = x_min as f64 / step;
    let x_max: f64 = x_max as f64 / step;

    let range = (x_min as i32..=x_max as i32).map(move |i| {
        let x: f64 = i as f64 * step;
        (x, a * x.powf(2.0) + b * x + c)
    });
    range
}

pub fn f3_deg(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    step: f64,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    res_mult: f64,
) -> () {
    let range = f3_deg_to_range(a, b, c, d, step, x_min, x_max);
    let range_style: ShapeStyle = line1_style(res_mult);

    let mut fn_str = format!("{a}x³");

    if b != 0.0 {
        fn_str = format!("{fn_str} + {b}x²");
    }
    if c != 0.0 {
        fn_str = format!("{fn_str} + {c}x");
    }
    if d != 0.0 {
        fn_str = format!("{fn_str} + {d}");
    }

    let _ = draw_plot_x640y480(
        x_min,
        x_max,
        y_min,
        y_max,
        range,
        range_style,
        fn_str,
        res_mult,
    );
}

pub fn f3_deg_easy(a: f64, b: f64, c: f64, d: f64, x_min: i32, x_max: i32, res_mult: f64) {
    let range = f3_deg_to_range(a, b, c, d, 0.01f64, x_min, x_max);
    let range_style = line1_style(res_mult);

    let mut fn_str: String = format!("{a}x³");

    if b != 0.0 {
        fn_str = format!("{fn_str} + {b}x²");
    }
    if c != 0.0 {
        fn_str = format!("{fn_str} + {c}x");
    }
    if d != 0.0 {
        fn_str = format!("{fn_str} + {d}");
    }

    let coll: Vec<_> = range.collect();

    let mut y_min: f64 = -5f64;
    let mut y_max: f64 = 5f64;
    for (_, y) in &coll {
        if y < &y_min {
            y_min = *y;
        }
        if y > &y_max {
            y_max = *y;
        }
    }
    let _ = draw_plot_x640y480(
        x_min,
        x_max,
        y_min as i32,
        y_max as i32,
        coll.into_iter(),
        range_style,
        fn_str,
        res_mult,
    );
}

pub fn f2_deg_easy(a: f64, b: f64, c: f64, x_min: i32, x_max: i32, res_mult: f64) {
    let range = f2_deg_to_range(a, b, c, 0.01f64, x_min, x_max);
    let range_style = line1_style(res_mult);

    let mut fn_str: String = format!("{a}x²");

    if b != 0.0 {
        fn_str = format!("{fn_str} + {b}x")
    }
    if c != 0.0 {
        fn_str = format!("{fn_str} + {c}")
    }

    let coll: Vec<_> = range.collect();

    let mut y_min: f64 = -5f64;
    let mut y_max: f64 = 5f64;
    for (_, y) in &coll {
        if y < &y_min {
            y_min = *y;
        }
        if y > &y_max {
            y_max = *y;
        }
    }
    let _ = draw_plot_x640y480(
        x_min,
        x_max,
        y_min as i32,
        y_max as i32,
        coll.into_iter(),
        range_style,
        fn_str,
        res_mult,
    );
}

pub fn f2_deg(
    a: f64,
    b: f64,
    c: f64,
    step: f64,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    res_mult: f64,
) -> () {
    // Função de segundo grau

    let range = f2_deg_to_range(a, b, c, step, x_min, x_max);
    let range_style: ShapeStyle = line1_style(res_mult);

    let mut fn_str: String = format!("{a}x²");

    if b != 0.0 {
        fn_str = format!("{fn_str} + {b}x")
    }
    if c != 0.0 {
        fn_str = format!("{fn_str} + {c}")
    }

    let _ = draw_plot_x640y480(
        x_min,
        x_max,
        y_min,
        y_max,
        range,
        range_style,
        fn_str,
        res_mult,
    );
    // chart.draw_series(LineSeries::new(range, line1_style.clone()))?;
}

fn draw_plot_x640y480(
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    range: impl Iterator<Item = (f64, f64)>,
    range_style: ShapeStyle,
    fn_str: String,
    res_mult: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(
        "plot.png",
        ((640f64 * res_mult) as u32, (480f64 * res_mult) as u32),
    )
    .into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(
        (10f64 * res_mult) as i32,
        (20f64 * res_mult) as i32,
        (30f64 * res_mult) as i32,
        (20f64 * res_mult) as i32,
    );

    let mut chart = ChartBuilder::on(&root)
        .caption(fn_str, ("sans-serrif", 40f64 * res_mult))
        .x_label_area_size((10f64 * res_mult) as i32)
        .y_label_area_size((30f64 * res_mult) as i32)
        .build_cartesian_2d(x_min as f64..x_max as f64, y_min as f64..y_max as f64)?;

    chart
        .configure_mesh()
        .label_style(("sans-serif", (18f64 * res_mult)).into_font())
        .x_labels(31) // Número de labels no eixo X (-10 a 10, incrementando de 1 em 1)
        .y_labels(31) // Número de labels no eixo Y (-10 a 200, incrementando de 10 em 10)
        .x_label_formatter(&|v| format!("{:.0}", v)) // Formata para inteiro
        .y_label_formatter(&|v| format!("{:.0}", v))
        .axis_style(ShapeStyle {
            color: RGBAColor(0, 0, 0, 1.0),
            filled: false,
            stroke_width: (1f64 * res_mult) as u32,
        })
        .max_light_lines(1)
        .bold_line_style(ShapeStyle {
            color: RGBAColor(0, 0, 50, 0.55),
            filled: false,
            stroke_width: (1f64 * res_mult) as u32,
        })
        .draw()?;

    let x_axis = (-1..=1).map(|x| {
        let x = x as f64 * 1000000f64;
        (x, 0.0f64)
    });

    let y_axis = (-1..=1).map(|y| {
        let y = y as f64 * 1000000f64;
        (0.0f64, y)
    });

    let xy_axis_style: ShapeStyle = ShapeStyle {
        color: RGBAColor(0, 0, 0, 1.0),
        filled: false,
        stroke_width: (2f64 * res_mult) as u32,
    };

    chart.draw_series(LineSeries::new(x_axis, xy_axis_style.clone()))?;
    chart.draw_series(LineSeries::new(y_axis, xy_axis_style.clone()))?;

    chart.draw_series(LineSeries::new(range, range_style))?;

    root.present()?;
    Ok(())
}
