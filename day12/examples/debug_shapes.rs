use day12::aoc_parser::AocParser;

fn main() {
    let input = include_str!("../puzzle-input.txt");
    let mut parser = AocParser::new();

    // Parse shapes (but not regions)
    // Actually, let me parse the whole thing
    match parser.parse(input) {
        Ok(regions) => {
            println!("Parsed {} regions", regions.len());

            // Get shapes from parser (need to make shapes public or add getter)
            // Actually, shapes are private. Let me check the shape definitions differently.
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }

    // Instead, let me manually parse the shapes from the input
    println!("\n--- Shape definitions from puzzle-input.txt ---");
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let mut shape_count = 0;

    while i < lines.len() && shape_count < 6 {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.ends_with(':') {
            let index_str = &line[..line.len() - 1];
            if let Ok(index) = index_str.parse::<usize>() {
                if index <= 5 {
                    println!("\nShape {}:", index);
                    i += 1;
                    let mut shape_lines = Vec::new();
                    while i < lines.len() && !lines[i].trim().is_empty() {
                        shape_lines.push(lines[i].trim());
                        i += 1;
                    }

                    for (y, shape_line) in shape_lines.iter().enumerate() {
                        println!("  {}", shape_line);
                    }
                    shape_count += 1;
                }
            }
        }
        i += 1;
    }
}
