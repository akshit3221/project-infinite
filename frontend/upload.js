$(document).ready(function() {
    $('#file').on('change', function() {
        var files = $(this).get(0).files;
        var fileList = $('#file-list');
        fileList.empty();

        if (files.length > 0) {
            $.each(files, function(index, file) {
                fileList.append('<p>' + file.name + '</p>');
            });
        } else {
            fileList.append('<p>No files selected</p>');
        }
    });

    $('.file-label').on('click', function() {
        $('#file').click();
    });
});
