Adder:
<expr> :=
  | <number>
  | (add1 <expr>)
  | (sub1 <expr>)
  | (negate <expr>)

Boa:
  <expr> :=
  | <number>
  | <identifier>
  | (let (<binding> +) <expr>)
  | (add1 <expr>)
  | (sub1 <expr>)
  | (+ <expr> <expr>)
  | (- <expr> <expr>)
  | (* <expr> <expr>)
