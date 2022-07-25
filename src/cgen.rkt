#lang racket

(define space #\space)  
(define tab #\tab)
(define return #\newline)

(define-syntax-rule (binary-expr op lhs rhs)
  (string lhs space op space rhs))

(define-syntax-rule (add lhs rhs)
  (binary-expr #\+ lhs rhs))

(define-syntax-rule (sub lhs rhs)
  (binary-expr #\- lhs rhs))

(define-syntax-rule (mult lhs rhs)
  (binary-expr #\* lhs rhs))

(define-syntax-rule (div lhs rhs)
  (binary-expr #\/ lhs rhs))



