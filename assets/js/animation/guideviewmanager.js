$(document).ready(function() {
    

});

function GuideViewManager() {
    this.refresh = GuideViewManagerRefresh;

    this.update = GuideViewManagerUpdate;
    this.draw = GuideViewManagerDraw;

    this.add_frame = GuideViewManagerAddFrame;

    this.frames = [];

    this.refresh();
}

function GuideViewManagerRefresh() {
    if(window.view_guid === undefined || window.view_guid === null || window.view_guid === ""){
        // Do nothing there is no view attached
        return;
    }

    this.frames = [];

     // Load images from view
     var url = "/api/view/" + window.view_guid;
     $.ajax({
         url: url,
         type: 'GET',
         dataType: 'json',
         success: function(ret){
             if(ret.status != 'ok'){
                 $('#error').html(ret.message);
                 return;
             }
             for(var i=0;i<ret.frames.length;i++){
                GUIDEVIEW_MANAGER.add_frame(ret.frames[i]);
             }
         },
         error: function(ret){
             console.log("ERROR creating new collection");
             console.log(ret);
         }
     })
}

function GuideViewManagerUpdate(perc) {
    // Do updates - calculate current frame showing
    this.cur_sel_frame = Math.floor(this.frames.length * perc)
}

function GuideViewManagerDraw(ctx) {
    if(this.cur_sel_frame === undefined || this.cur_sel_frame === null || this.cur_sel_frame >= this.frames.length) {
        return;
    }
    var cur_frame = this.frames[this.cur_sel_frame].image;
    if(cur_frame === undefined || cur_frame.width === undefined){
        return; // Not ready yet.
    }
    // TODO: Stretch or compress image as required
    ctx.drawImage(
        cur_frame,
        0,
        0,
        cur_frame.width,
        cur_frame.height
    );
}

function GuideViewManagerAddFrame(frame){
    var nxt = {
        image: new Image(),
        frame: frame
    }
    this.frames.push(nxt);
    this.frames[this.frames.length-1].image.src = "/img/viewframe/" + window.view_guid + "/" + frame;
}