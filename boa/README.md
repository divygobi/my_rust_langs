[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/P5qpkKKh)

The cfg here is given by

<expr> :=
  | <number>
  | <identifier>
  | (let (<binding> +) <expr>)
  | (add1 <expr>)
  | (sub1 <expr>)
  | (+ <expr> <expr>)
  | (- <expr> <expr>)
  | (* <expr> <expr>)

<binding> := (<identifier> <expr>)
