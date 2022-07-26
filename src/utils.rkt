#lang racket


;; converts a list of basically anything into the chars that make up the list
;; probably really not safe, the use could technically throw a s-expr into a spice
;; script and write something like (nested + 1 1)
;; now that example is harmless but it could be worse
;; I supppose the user should be responsible for handling that responsibly if they are
;; aware of it
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
       ((symbol=? (car lst) 'nest) (to-chars (eval (cdr lst))))
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

(provide (all-defined-out))
