on('#ajax', 'click', '.c_post_edit', function() {
  block = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  if (block.querySelector(".post_edit_form")) {
    return
  } else {
    clear_attach_block();
    div = document.createElement("div");
    block.append(div);
    block.querySelector(".fullscreen") ? block.querySelector(".fullscreen").style.display = "none" : null;
    block.querySelector(".attach_container") ? block.querySelector(".attach_container").style.display = "none" : null;
    block.querySelector(".card-footer").style.display = "none";
    pk = block.getAttribute("data-pk");
    list_load(div, "/posts/community_progs/edit_post/" + pk + "/");
  }
});

on('#ajax', 'click', '.c_article_detail', function() {
  var uuid, pk, loader;
  uuid = this.parentElement.getAttribute('data-uuid');
  document.body.querySelector(".pk_saver") ? pk = document.body.querySelector(".pk_saver").getAttribute('data-pk') : pk = this.parentElement.getAttribute('data-pk');
  create_fullscreen("/article/read/" + pk + "/" + uuid + "/", "item_fullscreen");
});


on('#ajax', 'click', '.c_comment_photo', function() {
  this.classList.add("current_file_dropdown");
  document.body.querySelector(".attach_block") ? (attach_block = document.body.querySelector(".attach_block"), attach_block.innerHTML = "", attach_block.classList.remove("attach_block")) : null;
  create_fullscreen('/users/load/c_img_comment_load/', "photo_fullscreen");
});
on('#ajax', 'click', '.c_comment_video', function() {
  this.classList.add("current_file_dropdown");
  clear_attach_block();
  create_fullscreen('/users/load/c_video_load/', "video_fullscreen");
});
on('#ajax', 'click', '.c_comment_music', function() {
  this.classList.add("current_file_dropdown");
  clear_attach_block();
  create_fullscreen('/users/load/c_music_load/', "worker_fullscreen");
});
on('#ajax', 'click', '.c_comment_good', function() {
  this.classList.add("current_file_dropdown");
  clear_attach_block();
  create_fullscreen('/users/load/c_good_load/', "item_fullscreen");
});
on('#ajax', 'click', '.c_comment_article', function() {
  this.classList.add("current_file_dropdown");
  clear_attach_block();
  create_fullscreen('/users/load/c_article_load/', "item_fullscreen");
});

on('#ajax', 'click', '.c_select_photo', function() {
  this.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/c_img_load/', "item_fullscreen");
});
on('#ajax', 'click', '.c_select_video', function() {
  this.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/c_video_load/', "item_fullscreen");
});
on('#ajax', 'click', '.c_select_music', function() {
  this.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/c_music_load/', "item_fullscreen");
});
on('#ajax', 'click', '.c_select_good', function() {
  this.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/c_good_load/', "item_fullscreen");
});
on('#ajax', 'click', '.c_select_article', function() {
  this.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/c_article_load/', "item_fullscreen");
});
