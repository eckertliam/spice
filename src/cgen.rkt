#lang racket

(define (to-chars lst)
  (cond
    ((null? lst) '())
    ((number? lst) (string->list (number->string lst)))
    ((string? lst) (string->list lst))
    ((number? (car lst))
     (append
      (string->list (number->string (car lst)))
      (to-chars (cdr lst))))
    ((symbol? (car lst))
     (cond
       ((symbol=? (car lst) 'safe) (to-chars (eval (cdr lst))))
       (else (append
              (string->list (symbol->string (car lst)))
              (to-chars (cdr lst))))))
    ((string? (car lst)) (append
                          (string->list (car lst))
                          (to-chars (cdr lst))))
    ((list? (car lst))
     (append
      (to-chars (car lst))
      (to-chars (cdr lst)))) 
    (else (cons
           (car lst)
           (to-chars (cdr lst))))))

(define (binary-c-expr lst)
  (list->string (to-chars lst)))

(define-syntax-rule (c-add lhs rhs)
  (binary-c-expr '(lhs #\space + #\space rhs)))

(define-syntax-rule (c-sub lhs rhs)
  (binary-c-expr '(lhs #\space - #\space rhs)))

(define-syntax-rule (c-mult lhs rhs)
  (binary-c-expr '(lhs #\space * #\space rhs)))

(define-syntax-rule (c-div lhs rhs)
  (binary-c-expr '(lhs #\space / #\space rhs)))

(define-syntax-rule (c-equal lhs rhs)
  (binary-c-expr '(lhs #\space == #\space rhs)))

(define-syntax-rule (c-or lhs rhs)
  (binary-c-expr '(lhs #\space || #\space rhs)))

(define-syntax-rule (c-and lhs rhs)
  (binary-c-expr '(lhs #\space && #\space rhs)))

