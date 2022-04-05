on('#ajax', 'click', '.add_video', function() {
  create_fullscreen("/video/add_video_in_list/" + this.parentElement.parentElement.getAttribute("data-pk") + "/", "worker_fullscreen");
});
on('#ajax', 'click', '.uri_click', function() {
  this.nextElementSibling.style.display = "block";
  this.parentElement.nextElementSibling.querySelector("#add_video_btn").style.display = "block";
  fullscreen_resize()
});
on('#ajax', 'click', '.load_profile_video_list', function() {
  profile_list_block_load(this, ".load_block", "/video_list/", "load_profile_video_list");
});
on('#ajax', 'click', '.load_attach_video_list', function() {
  profile_list_block_attach(this, ".load_block", "/u_video_list_load/", "load_attach_video_list");
});

on('#ajax', 'click', '.load_video_list', function() {
  if (this.getAttribute("videolist-pk")) {
    videolist_pk = this.getAttribute("videolist-pk");
    owner_pk = null
  } else {
    card = this.parentElement.parentElement.parentElement;
    videolist_pk = card.getAttribute("videolist-pk");
    owner_pk = card.getAttribute("owner-pk");
  };

  create_fullscreen("/video/load_list/" + videolist_pk + "/", "item_fullscreen");
  if (owner_pk) {
    window.history.pushState(null, "vfgffgfgf", window.location.href + "?key=wall&owner_id=" + owner_pk + "&videolist=" + videolist_pk);
  }
});

on('#ajax', 'click', '.video_list_detail', function() {
  video_pk = this.getAttribute("video-pk");
  counter = this.getAttribute('video-counter') - 1;
  play_video_list("/video/user/list/" + video_pk + "/", counter, video_pk)
});

on('#ajax', 'click', '.post_video', function() {
  video_pk = this.getAttribute("video-pk");
  pk = this.parentElement.parentElement.parentElement.getAttribute("data-pk");
  counter = this.getAttribute('video-counter') - 1;
  play_video_list("/video/user/list_post/" + pk + "/", counter, video_pk)
});

on('#ajax', 'click', '.message_video', function() {
  video_pk = this.getAttribute("video-pk");
  uuid = this.parentElement.parentElement.parentElement.parentElement.getAttribute("data-uuid");
  counter = this.getAttribute('video-counter') - 1;
  play_video_list("/video/user/list_message/" + uuid + "/", counter, video_pk)
});

on('#ajax', 'click', '.play_comment_video', function() {
  comment_pk = this.getAttribute("comment-pk");
  video_pk = this.getAttribute("video-pk");
  counter = this.getAttribute('video-counter') - 1;
  play_video_list("/video/user/list_post_comment/" + video_pk + "/", counter, video_pk);
});

on('body', 'click', '.video_fullscreen_resize', function() {
  video_window = document.querySelector(".video_fullscreen");
  video_window.classList.add("video_fullscreen_resized", "draggable");
  document.body.querySelector(".video_btn_big").style.display = "none";
  document.body.querySelector(".video_btn_small").style.display = "block";
  get_resize_screen();
  dragElement(document.querySelector(".draggable"));

});
on('body', 'click', '.video_fullscreen_normal', function() {
  video_window = document.querySelector(".video_fullscreen");
  video_window.style.top = "0"; video_window.style.left = "auto";
  video_window.classList.remove("video_fullscreen_resized", "draggable");
  document.body.querySelector(".video_btn_small").style.display = "none";
  document.body.querySelector(".video_btn_big").style.display = "block";
  get_normal_screen()
});

on('body', 'click', '#video_holder', function() {
  ggg = this;
  img = this.previousElementSibling.querySelector("#id_image");
  get_image_priview(ggg, img)
});
