#lang racket

(require "utils.rkt")

(define-syntax-rule (c-expr lst)
  (list->string (to-chars lst)))

(define-syntax-rule (binary-c-expr lhs op rhs)
  (c-expr '(lhs #\space op #\space rhs)))

(define-syntax-rule (assign-val type name val)
  (list->string (to-chars '(type #\space name #\space #\= #\space val))))

(define-syntax-rule (c-add lhs rhs)
  (binary-c-expr lhs + rhs))

(define-syntax-rule (c-sub lhs rhs)
  (binary-c-expr lhs - rhs))

(define-syntax-rule (c-mult lhs rhs)
  (binary-c-expr lhs * rhs))

(define-syntax-rule (c-div lhs rhs)
  (binary-c-expr lhs / rhs))

(define-syntax-rule (c-equal lhs rhs)
  (binary-c-expr lhs == rhs))

(define-syntax-rule (c-or lhs rhs)
  (binary-c-expr lhs || rhs))

(define-syntax-rule (c-and lhs rhs)
  (binary-c-expr lhs && rhs))

(define-syntax-rule (c-if-expr conditonal body)
  (list->string (to-chars '("if" #\space conditonal #\space #\{ #\newline
                                 body
                                 #\newline #\} #\newline))))

