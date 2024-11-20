"what operator do you wanna do"?
"please choose one of the followig operators"?
cout("+, -, * or /: ")

op = cin_char()

cout("num 1 is: ")
x = cin_number()

cout("num 2 is: ")
y = cin_number()

map op {
  "+" => x + y
  "-" => x - y
  "*" => x * y
  "/" => x / y
  _ => "no"
}?
