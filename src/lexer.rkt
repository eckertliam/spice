#lang racket

(require "utils.rkt")

(define (read-file fname)
  (string->list (file->string fname)))

;; so that I can use symbols almost like enums without worrying about using symbol=? and any other mangling that could occur to a symbol
(define-syntax-rule (token-t t)
  (symbol->string t))

;; lexical tokens contain the name value and type of the token
;; types for tokens denoted in TOKEN-T constant
(struct token (name value type))

;; grabs the next atom from a list of chars
(define (next-atom chars)
  (cond
    ((null? chars) (error "Unexpected EOF"))
    ((or (equal? #\space (car chars))
         (equal? #\tab (car chars))
         (equal? #\newline (car chars)))
     '())
    (else (cons (car chars) (next-atom (cdr chars))))))

(define (operation? op)
  (member op '(#\* #\/
               #\+ #\-
               #\> #\<
               #\= #\|)))

(define (lex-keyword? key)
  (member key '("let" "cond" "print" "read" "eval"
                      "fn" "type" "symbol" "lambda")))

(define (token-t? t)
  (member t '("literal" "id" "operation" "keyword" "seperator" "eof" "whitespace")))


(define (whitespace? q)
  (member q '(#\space #\newline #\tab)))

(define (eof? e)
  (eof-object? e))

(define (seperator? sep)
  (member sep '(#\{ #\}
                #\( #\)
                #\[ #\]
                #\" #\')))

(define (id? id)
  (cond
    ((not (list? id)) (id? (to-chars id)))
    ((null? id) #t)
    ((not (char? (car id))) (id? (to-chars id)))
    ((seperator? (car id)) #f)
    (else (id? (cdr id)))))

(define (literal? lit)
  (cond
    ((null? lit) #t)
    ((not (list? lit)) (literal? (to-chars lit)))
    ((not (char? (car lit))) (literal? (to-chars lit)))
    ((or (seperator? (car lit))
         (lex-keyword? (car lit))
         (id? (car lit))
         (operation? (car lit))))
    (else (literal? (cdr lit)))))

(provide (all-defined-out))
