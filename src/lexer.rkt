#lang racket

(define (read-file fname)
  (string->list (file->string fname)))

;; lexical tokens contain the name value and type of the token
;; types for tokens denoted in TOKEN-T constant
(struct token (name value type))

;; available types for tokens
(define TOKEN-T
  (lambda () '(operand literal identifier endline endfile keyword seperator tab)))

;; member implemented for a list of symbols using symbol=? in place of eq?
(define (member-symbol? sym lst)
  (cond
    ((not (symbol? sym)) #f)
    ((null? lst) #f)
    ((symbol=? sym (car lst)) #t)
    (else (member-symbol? sym (cdr lst)))))

;; is a symbol and is a member of the token type set
(define (token-t? t)
  (cond
    ((not (symbol? t)) #f)
    ((member-symbol? t (TOKEN-T)) #t)
    (else #f)))

;; grabs the next atom from a list of chars
(define (next-atom chars)
  (cond
    ((null? chars) (error "Unexpected EOF"))
    ((or (equal? #\space (car chars))
         (equal? #\tab (car chars))
         (equal? #\newline (car chars)))
     '())
    (else (cons (car chars) (next-atom (cdr chars))))))

(provide (all-defined-out))
