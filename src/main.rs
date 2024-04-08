const SVG: &str = r#"
  <svg viewBox="-5 -5 10 10" xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
    <g transform="scale(1 -1)">
      <polygon points="0.5,-0.5 0.5,0.5 -0.5,0.5 -0.5,-0.5" stroke-width="0.2%" stroke="black" fill="lightgrey" />
    </g>
  </svg>
"#;

fn main() {
  println!("{}", SVG);
}