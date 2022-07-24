#lang racket

(require rackunit)

;; tests for lexer.rkt
(require "lexer.rkt")

(check-equal? (next-atom (string->list "this is a test")) '(#\t #\h #\i #\s))
