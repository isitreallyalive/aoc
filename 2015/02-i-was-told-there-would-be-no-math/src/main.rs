const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut paper = 0;
    let mut ribbon = 0;

    for present in INPUT.lines() {
        // read the present's dimensions
        let [l, h, w] = present
            .split('x')
            .map(|x| x.parse())
            .collect::<Result<Vec<u16>, _>>()
            .expect("input should be valid")[..]
        else {
            continue;
        };

        // compute the areas of the unique sides
        let a = l * h;
        let b = l * w;
        let c = h * w;

        // find the surface area and area of the minimum side
        let required = 2 * (a + b + c);
        let slack = a.min(b).min(c);

        paper += (required + slack) as u32;

        // now work out the ribbon
        let wrap_present = {
            let mut dims = [l, h, w];
            dims.sort();
            2 * (dims[0] + dims[1])
        };
        let bow = l * h * w;

        ribbon += (wrap_present + bow) as u32;
    }

    println!(
        "The elves should order a total of {paper} square feet of wrapping paper, and {ribbon} feet of ribbon.",
    );
}
