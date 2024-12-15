var anim_list = [];
var search_prompt = "";

$(document).ready(function(){

    var url = "/api/animation";
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        success: function(ret){
            console.log(ret);
            if(ret.status != "ok"){
                $('#error').html(ret.message);
                return;
            }
            anim_list = ret.animations;
            refresh_anim_list();
        },
        error: function(ret){
            console.log("Error getting saved pixels")
            console.log(ret)
        }
    })

})

function refresh_anim_list() {

    var output = "";
    for(var i=0;i<anim_list.length;i++){
        var p = anim_list[i];

        if(search_prompt !== "" && !p.name.includes(search_prompt)) {
            continue;
        }

        output += "<div class='row choose_menu_items'>"
        // output += "<a href='/pixelapp/" + p.id + "'>"
        output += "<div class='column choose_menu_item'>" + p.name + "<br>";
        output += "<br><img src='/img/animation_gif/" + p.guid + "' height='250'><br>"
        output += "<button onclick='load_animation(\""+ p.guid + "\")'>Open</button>";
        // output += "<button onclick='load_render_page(\"" + p.guid + "\")'>Render</button>";
        // output += "<button onclick='load_png_image(\"" + p.guid + "\")'>PNG</button>";
        // output += "<button onclick='show_delete_confirm(\"" + p.guid + "\")'>Delete</button>";
        // output += "<div id='confirm_delete_" + p.guid + "' style='display:none;'>";
        // output += "This process is unrecoverable are you sure?<br>"
        // output += "<button onclick='delete_pixel(\"" + p.guid + "\")'>Confirm</button>";
        // output += "<button onclick='cancle_confirmation(\"" + p.guid + "\")'>Cancel</button>";
        // output += "</div>"
        output += "</div>";
        // output += "</a>";
        output += "</div>";

    }
    
    $('#saved_items').html(output);
}

function load_animation(id){
    window.location.href='/animation/' + id;
}

// function load_render_page(id){
//     window.location.href='/render/' + id;
// }

// function load_png_image(id){
//     window.location.href='/img/' + id + '/png'
    
// }

// function show_delete_confirm(pid){
//     $('#confirm_delete_'+ pid).show();
// }
// function cancle_confirmation(pid){
//     $('#confirm_delete_'+ pid).hide();
// }
// function delete_pixel(pid){
//     var url = "/api/" + pid;
//     console.log(url);

//     $.ajax({
//         url: url,
//         type: 'DELETE',
//         dataType: 'json',
//         // beforeSend: function (xhr) {
//         //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
//         // },
//         success: function(ret){
//             // console.log(ret);
//             if(ret.status != "ok"){
//                 console.log(ret.message);
//                 $('#error').html(ret.message);
//                 return;
//             }
            
//             window.location.reload();
          
//         },
//         error: function(ret){
//             console.log("ERROR while deleting saved data");
//             console.log(ret);
//         }
//       })
// }

function update_search_prompt() {
    var val = $('#search').val();
    search_prompt = val;

    refresh_anim_list();
}