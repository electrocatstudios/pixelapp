function upload_video_file(){
    let file = $("#filename")[0].files[0];
    let reader = new FileReader();
    
    var name = $('#name').val();
    var desc = $('#description').val();

    // Closure to capture the file information.
    reader.onload = (function(filename) {
      return function(e) {
        let bytes = e.target.result; 

        var fd = new FormData();
        fd.append("name", name);
        fd.append("description", desc);
        fd.append("file", new Blob([bytes], {type:"video/mp4"}));

        let url = "/api/video_upload";
        $.ajax({
            url: url,
            type: 'POST',
            data: fd,
            contentType: false,
            processData: false,
            // contentType: "video/mp4",
            // beforeSend: function (xhr) {
            //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
            // },
            success: function(ret){
                if(ret.status != 'ok'){
                    $('#errorfileupload').html(ret.message);
                    return;
                }
                // console.log(ret);
                window.location.href='/';
            },
            error: function(ret){
                console.log("ERROR while uploading video");
                console.log(ret);
            }
        })
      };
    })(file);
    
    reader.readAsArrayBuffer(file);
}