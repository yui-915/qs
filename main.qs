"welcome yui's calculator"?
"written in my own programming language!!"?
""?

let op
while op != "+" && op != "-" && op != "*" && op != "/" {
  cout("please select one of (+ - * /): ")
  op = cin_char()
}

x = read_num("1")
y = read_num("2")

map op {
  "+" => x + y
  "-" => x - y
  "*" => x * y
  "/" => x / y
}?

fn read_num(n) {
  let output
  while output == nil {
    cout("num " + n + " is: ")
    output = cin_number()
  }
}
