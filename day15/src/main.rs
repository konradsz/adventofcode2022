#[derive(Debug, Clone, Copy)]
struct Report {
    sensor_x: i64,
    sensor_y: i64,
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
    if rhs.start - 1 <= lhs.end {
        Some(Range {
            start: std::cmp::min(lhs.start, rhs.start),
            end: std::cmp::max(lhs.end, rhs.end),
        })
    } else {
        None
    }
}

fn part_1(reports: &[Report]) -> i64 {
    const ROW: i64 = 2000000;
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

fn part_2(reports: &[Report]) -> i64 {
    for row in 0..=4000000 {
        let mut ranges = vec![];
        for report in reports {
            let cov = report.get_coverage_at_row(row);

            if let Some(range) = cov {
                ranges.push(range);
            }
        }

        merge_ranges(&mut ranges);
        if ranges.len() != 1 {
            return (ranges[0].end + 1) * 4000000 + row;
        }
    }
    panic!()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
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

        let r = Report {
            sensor_x,
            sensor_y,
            radius: (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs(),
        };
        reports.push(r);
    }

    assert_eq!(part_1(&reports), 5564017);
    assert_eq!(part_2(&reports), 11558423398893);
}
