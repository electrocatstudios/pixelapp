var is_loading = false;

function create_collection(){
    if(is_loading === true) {
        console.log("Still waiting for previous run to finish");
        return;
    }

    $('#error').html("");

    var coll_name = $(collection).val().trim();
    if(coll_name == "") {
        $('#error').html("Collection name cannot be blank");
        return;
    }
    var data = {
        collection_name: coll_name
    };

    var url = "/api/collection";
    $.ajax({
        url: url,
        type: 'POST',
        dataType: 'json',
        contentType: "application/json; charset=utf-8",
        data: JSON.stringify(data),
        // beforeSend: function (xhr) {
        //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
        // },
        success: function(ret){
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            window.location.href='/';
        },
        error: function(ret){
            console.log("ERROR creating new collection");
            console.log(ret);
        }
    })
}