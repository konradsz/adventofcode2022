use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Report {
    sensor_x: i64,
    sensor_y: i64,
    // beacon_x: i64,
    // beacon_y: i64,
    radius: i64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: i64,
    end: i64,
}

impl Report {
    fn get_coverage_at_row(self, y: i64) -> Option<Range> {
        let vertical_distance = (self.sensor_y - y).abs();
        // use then_some
        if vertical_distance > self.radius {
            None
        } else {
            Some(Range {
                start: self.sensor_x - (self.radius - vertical_distance),
                end: self.sensor_x + (self.radius - vertical_distance),
            })
        }
    }
}

fn merge_ranges(ranges: &mut Vec<Range>) {
    ranges.sort();

    'outer: loop {
        if ranges.len() == 1 {
            break;
        }

        for i in 0..ranges.len() - 1 {
            if let Some(merged) = merge(ranges[i], ranges[i + 1]) {
                let (left, right) = ranges.split_at(i);
                let (_, right) = right.split_at(2);
                *ranges = Vec::from_iter(
                    left.iter()
                        .copied()
                        .chain(std::iter::once(merged).chain(right.iter().copied())),
                );
                continue 'outer;
            }
        }

        break 'outer;
    }
}

fn merge(lhs: Range, rhs: Range) -> Option<Range> {
    if rhs.start <= lhs.end {
        Some(Range {
            start: std::cmp::min(lhs.start, rhs.start),
            end: std::cmp::max(lhs.end, rhs.end),
        })
    } else {
        None
    }
}

fn part_1(reports: &[Report]) -> i64 {
    // const ROW: i64 = 2000000;
    const ROW: i64 = 10;
    let mut ranges = vec![];
    for report in reports {
        let cov = report.get_coverage_at_row(ROW);

        if let Some(range) = cov {
            ranges.push(range);
        }
    }

    merge_ranges(&mut ranges);
    ranges[0].start.abs() + ranges[0].end.abs()
}
fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut beacon_at_asked = HashSet::new();
    let mut reports = vec![];
    for line in input.lines() {
        let mut s = line.split(&['=', ',', ':']);
        s.next();
        let sensor_x = s.next().unwrap().parse::<i64>().unwrap();
        s.next();
        let sensor_y = s.next().unwrap().parse::<i64>().unwrap();
        s.next();
        let beacon_x = s.next().unwrap().parse::<i64>().unwrap();
        s.next();
        let beacon_y = s.next().unwrap().parse::<i64>().unwrap();

        if beacon_y == 2000000 {
            beacon_at_asked.insert(beacon_x);
        }

        let r = Report {
            sensor_x,
            sensor_y,
            // beacon_x,
            // beacon_y,
            radius: (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs(),
        };
        reports.push(r);
    }

    // for report in reports {
    //     let cov = report.get_coverage_at_row(2000000);

    //     if let Some(range) = cov {
    //         ranges.push(range);
    //     }
    // }

    // merge_ranges(&mut ranges);
    // println!(
    //     "{ranges:?}, {}",
    //     ranges[0].start.abs() + ranges[0].end.abs()
    // );

    // assert_eq!(part_1(&reports), 5564017);
}
