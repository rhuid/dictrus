;;; dictrus.el --- A simple dictionary brought to Emacs -*- lexical-binding: t; -*-

(defun rh/dictrus-lookup (word)
  "Look up WORD using the dictrus CLI and display the result in a buffer."
  (interactive (list (read-string "Lookup word: " (thing-at-point 'word t))))
  (let* ((buf (get-buffer-create "*Dictrus*"))
         (result (with-temp-buffer
                   (call-process "dictrus" nil t nil word)
                   (buffer-string))))
    (with-current-buffer buf
      (read-only-mode -1)
      (erase-buffer)
      (insert (format "Word: %s\n\n%s" word result))
      (goto-char (point-min))
      (view-mode 1)) ;; view-mode for easy quit
    (pop-to-buffer buf)))

(provide 'dictrus)
