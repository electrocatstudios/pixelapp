$(document).ready(function() {
    $('#delete_confirm').addClass('delete_hidden');

    var url = "/api/view";
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        success: function(ret){
            if(ret.status != "ok"){
                $('#error').html(ret.message);
                return;
            }
            var output = "";
            for(var i=0;i<ret.views.length;i++){
                let v = ret.views[i];
                output += "<div class='row choose_menu_items'>";
                output += "<div class='column choose_menu_item'><p>Name: " + v.name + "</p>";
                output += "<p>Guid: " + v.guid + "</p>";
                output += "<button onclick='load_view_page(\"" + v.guid + "\")'>Open</button>";
                output += "<button onclick='delete_animation(\""+ v.guid + "\")'>Delete</button>";
                output += "</div>";
                output += "</div>";
            }
            // console.log(output);
            $('#coll_container').html(output);
        },
        error: function(ret){
            console.log("Error getting saved videos")
            console.log(ret)
        }
    })

});

function load_view_page(guid) {
    window.location.href='/view_preview/' + guid;
}

var guid_to_delete = null;
function delete_animation(guid){
    guid_to_delete = guid;
    $('#delete_confirm').removeClass('delete_hidden');
}

function confirm_deletion(){
    var url = "/api/view/" + guid_to_delete;
    $.ajax({
        url: url,
        type: 'DELETE',
        dataType: 'json',
        success: function(ret){
            console.log(ret);
            if(ret.status !== "ok"){
                $('#error').html(ret.message);
                return;
            }
            guid_to_delete = null;
            $('#delete_confirm').addClass('delete_hidden');
            window.location.reload();
        },
        error: function(err) {
            console.log("Error deleting a stored view");
            console.log(err);
        }
    });
}

function cancel_deletion() {
    guid_to_delete = null;
    $('#delete_confirm').addClass('delete_hidden');
}