pub fn get_game_xml() -> Box<&'static str> {
    let data = r##"
    <games>
        <game>
            <name>Clock rotation</name>
            <type>Logic</type>
            <difficulty>All</difficulty>
            <variables>
                let nullm = (1 + Math.floor(Math.random() * 5)) * 10;
                let rslt = (2 * 360) + (num * 6);
            </variables>
            <rationale>Every hour rotates 360 degrees.</rationale>
            <svg file = "clock.svg" x = "0.25" y = "0.25" width = "0.5" height = "0.5"/>
            <question>How many degrees rotates the minute hand of a clock in 2 hours [num] minute?</question>
            <question plural ="[rslt]">How many degrees rotates the minute hand of a clock in 2 hours [num] minutes?</question>
            <answer>[rslt]</answer>
        </game>

        <game>
            <name>Simple equations</name>
            <type>Calculation</type>
            <difficulty>All</difficulty>
            <!-- Addition -->
            <variant>
                <variables>
                    let num_a = 30 + Math.floor((Math.random() * 20));
                    let num_b = 60 + Math.floor((Math.random() * 20));
                    let rslt = num_b - num_a;
                </variables>
                <question>What number plus [num_a] equals [num_b]?</question>
                <string text = "x + [num_a] = [num_b]" x = "0.5" y = "0.4" centered = "yes" size = "large"/>
                <answer>[rslt]</answer>
                <rationale>It is the result of the operation [num_b] - [num_a].</rationale>
            </variant>

            <!-- Subtraction -->
            <variant>
                <variables>
                    let num_a = 30 + Math.floor((Math.random() * 20));
                    let num_b = 60 + Math.floor((Math.random() * 20));
                    let rslt = num_b + num_a;
                </variables>
                <question>What number minus [num_a] equals [num_b]?</question>
                <string text = "x - [num_a] = [num_b]" x = "0.5" y = "0.4" centered = "yes" size = "large"/>
                <answer>[rslt]</answer>
                <rationale>It is the result of the operation [num_a] + [num_b].</rationale>
            </variant>

        </game>
    </games>
    "##;
    Box::new(data)
}
