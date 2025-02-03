const textArea = document.getElementById('page-edit-textarea');
if (textArea !== null) {
    textArea.addEventListener('keydown', function (e) {
        if (e.key === 'Tab') {
            // insert tab instead of switching selection
            e.preventDefault();
            document.execCommand('insertText', false, '    ');
        }
    })
}
