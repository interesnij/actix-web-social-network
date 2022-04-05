on('body', 'click', '.load_comments_list', function() {
  clear_comment_dropdown();
  block = this.parentElement.parentElement.parentElement.parentElement;
  block_comments = block.querySelector(".load_comments");
  if (block_comments.classList.contains("show")){
    block_comments.classList.remove("show")
  } else {
    block_comments.firstChild
        ? null
        : list_load(block_comments, "/comments/list/?type=" + this.parentElement.getAttribute("data-type"));
    block_comments.classList.add("show")
  }
});

on('#ajax', 'click', '.load_posts_list', function() {
  if (this.getAttribute("postlist-pk")) {
    postlist_pk = this.getAttribute("postlist-pk");
    owner_pk = null
  } else {
    card = this.parentElement.parentElement.parentElement;
    postlist_pk = card.getAttribute("postlist-pk");
    owner_pk = card.getAttribute("owner-pk");
  };

  create_fullscreen("/posts/list/?list=" + postlist_pk, "worker_fullscreen");
  if (owner_pk) {
    window.history.pushState(null, "vfgffgfgf", window.location.href + "?key=wall&owner_id=" + owner_pk + "&postlist=" + postlist_pk);
  }
});

on('body', 'click', '.create_repost', function() {
  parent = this.parentElement;
  type = parent.getAttribute('data-type');
  if (parent.getAttribute('data-subtype')) {
    subtype = parent.getAttribute('data-subtype')
  } else { subtype = null};
  create_fullscreen("/users/progs/create_repost/?type=" + type, "worker_fullscreen");
  clear_attach_block();
});

on('body', 'click', '.create_claim', function() {
  parent = this.parentElement;
  type = parent.getAttribute('data-type');
  dropdowns = document.body.querySelectorAll(".dropdown-menu");
  for (var i = 0; i < dropdowns.length; i++) {
    dropdowns[i].classList.remove("show")
  };
  create_fullscreen("/users/progs/create_claim/?type=" + type, "worker_fullscreen");
});

on('body', 'click', '.create_list', function() {
  parent = this.parentElement;
  type = parent.getAttribute('data-type');
  community_id = parent.getAttribute('data-community-id');
  create_fullscreen("/users/progs/create_list/?type=" + type + "&community_id=" + community_id, "worker_fullscreen");
});
on('body', 'click', '.edit_list', function() {
  parent = this.parentElement;
  type = parent.getAttribute('data-type');
  community_id = parent.getAttribute('data-community-id');
  create_fullscreen("/users/progs/edit_list/?type=" + type + "&community_id=" + community_id, "worker_fullscreen");
});

on('body', 'click', '.comment_likes', function() {
  create_fullscreen("/comments/likes/?type=" + this.parentElement.parentElement.parentElement.getAttribute("data-type"), "worker_fullscreen");
});
on('#ajax', 'click', '.comment_dislikes', function() {
  create_fullscreen("/comments/dislikes/?type=" + this.parentElement.parentElement.parentElement.getAttribute("data-type"), "worker_fullscreen");
});

on('body', 'click', '.item_likes', function() {
  create_fullscreen("/items/likes/?type=" + this.parentElement.parentElement.parentElement.getAttribute("data-type"), "worker_fullscreen");
});
on('body', 'click', '.item_dislikes', function() {
  create_fullscreen("/items/dislikes/?type=" + this.parentElement.parentElement.parentElement.getAttribute("data-type"), "worker_fullscreen");
});

on('#ajax', 'click', '.input_new_post_in_list', function() {
  this.parentElement.nextElementSibling.style.display = "block";
});

on('#ajax', 'click', '.post_list_change', function() {
  if (!this.classList.contains("tab_active")){
    parent = this.parentElement.parentElement.parentElement;
    list = parent.querySelectorAll(".list");
    for (var i = 0; i < list.length; i++) {
      list[i].classList.remove("active");
      list[i].classList.add("pointer", "post_list_change");
    };
    block = parent.nextElementSibling;
    list_block_load(block, ".span_list_pk", "/posts/list/?list=" + this.getAttribute("list-pk"));
    this.classList.remove("pointer", "post_list_change");
    this.classList.add("active");
    try{ reload_list_stat(this) }catch { null };
    get_dragula(".drag_container");
  }
});

on('#ajax', 'click', '.post_list_select', function() {
  parent = this.parentElement;
  lists = parent.parentElement.querySelectorAll(".post_list_select");
  for (var i = 0; i < lists.length; i++){
    lists[i].querySelector("svg") ? (lists[i].querySelector("svg").parentElement.remove(), lists[i].style.paddingLeft = "30px") : null;
  }
  pk = parent.getAttribute("data-pk");
  list = parent.querySelector(".post_list_select");
  list.style.paddingLeft = "14px",
  span = document.createElement("span"),
  span.innerHTML = '<input type="hidden" class="list" name="lists" value="' + pk + '"><svg fill="currentColor" style="width:12px;height:12px;" class="svg_default" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0z"/><path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z"/></svg> ',
  list.prepend(span)
});
on('#ajax', 'click', '.cat_list_select', function() {
  parent = this.parentElement;
  lists = parent.parentElement.querySelectorAll(".cat_list_select");
  for (var i = 0; i < lists.length; i++){
    lists[i].querySelector("svg") ? (lists[i].querySelector("svg").parentElement.remove(), lists[i].style.paddingLeft = "30px") : null;
  }
  pk = parent.getAttribute("data-pk");
  list = parent.querySelector(".cat_list_select");
  list.style.paddingLeft = "14px",
  span = document.createElement("span"),
  span.innerHTML = '<input type="hidden" name="cat" value="' + pk + '"><svg fill="currentColor" style="width:12px;height:12px;" class="svg_default" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0z"/><path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z"/></svg> ',
  list.prepend(span)
});

on('#ajax', 'click', '#holder_article_image', function() {
  img = this.previousElementSibling.querySelector("#id_g_image")
  get_image_priview(this, img);
});

on('#ajax', 'click', '.wall_fullscreen', function(e) {
  e.preventDefault();
  card = this.parentElement.parentElement.parentElement.parentElement;
  pk = card.getAttribute('data-pk');
  create_fullscreen("/posts/post/" + pk + "/", "worker_fullscreen");
  window.history.pushState(null, "vfgffgfgf", window.location.href + "?key=wall&owner_id=" + pk + "&post_pk=" + pk);
});

on('#ajax', 'click', '.fullscreen', function(e) {
  card = this.parentElement;

  if (this.parentElement.querySelector(".show_post_text")) {
    shower = this.parentElement.querySelector(".show_post_text");
    shower.nextElementSibling.nextElementSibling.style.display = "unset";
    shower.nextElementSibling.remove();
    shower.previousElementSibling.remove();
    shower.remove();
  }

  else if (e.target.classList.contains("action")) {null}
  else {
    pk = card.getAttribute('data-pk');
    create_fullscreen("/posts/post/" + pk + "/", "worker_fullscreen");
    window.history.pushState(null, "vfgffgfgf", window.location.href + "?key=wall&owner_id=" + pk + "&post_pk=" + pk);
  }
});

on('#ajax', 'click', '.fix_fullscreen', function(e) {
  card = this.parentElement;

  if (this.parentElement.querySelector(".show_post_text")) {
    shower = this.parentElement.querySelector(".show_post_text");
    shower.nextElementSibling.nextElementSibling.style.display = "unset";
    shower.nextElementSibling.remove();
    shower.previousElementSibling.remove();
    shower.remove();
  }

  else if (e.target.classList.contains("action")) {null}
  else {
    pk = card.getAttribute('data-pk');
    create_fullscreen("/posts/fix_post/" + pk + "/", "worker_fullscreen");
    window.history.pushState(null, "vfgffgfgf", window.location.href + "?key=wall&owner_id=" + pk + "&post_pk=" + pk);
  }
});

on('#ajax', 'click', '#toggle_case_item_repost', function() {
  this.nextElementSibling.classList.replace("underline", "pointer");
  this.classList.replace("pointer", "underline");
  btn = this.parentElement.parentElement.nextElementSibling.nextElementSibling.querySelector(".float-right");
  btn.removeAttribute("id");
  btn.setAttribute("id", this.getAttribute("data-flag"));
  btn.innerHTML = this.innerHTML;
  form = this.parentElement.parentElement.parentElement;
  form.querySelector("#repost_for_message").style.display = "unset";
  form.querySelector(".form_body").style.display = "block";
  form.querySelector(".collector_active").innerHTML = "";
  if (form.querySelector(".copy_case")) {
    form.querySelector(".repost_case").style.display = "block";
    form.querySelector(".copy_case").style.display = "none";
  }
});
on('#ajax', 'click', '#toggle_case_item_copy', function() {
  this.previousElementSibling.classList.replace("underline", "pointer");
  this.classList.replace("pointer", "underline");
  btn = this.parentElement.parentElement.nextElementSibling.nextElementSibling.querySelector(".float-right");
  btn.removeAttribute("id");
  btn.setAttribute("id", this.getAttribute("data-flag"));
  btn.innerHTML = this.innerHTML;
  form = this.parentElement.parentElement.parentElement;
  form.querySelector("#repost_for_message").style.display = "none";
  form.querySelector(".form_body").style.display = "none";
  form.querySelector(".collector_active").innerHTML = "";
  if (form.querySelector(".copy_case")) {
    form.querySelector(".repost_case").style.display = "none";
    form.querySelector(".copy_case").style.display = "block";
  }
});
on('#ajax', 'click', '#copy_for_profile', function() {
  this.querySelector(".copy_for_profile").setAttribute("checked", "true");
  parent = this.parentElement;
  parent.querySelector(".copy_for_communities").removeAttribute("checked");
  current_block = parent.nextElementSibling;
  current_block.querySelector(".collector").innerHTML = "";
});
on('#ajax', 'click', '#copy_for_communities', function() {
  this.querySelector(".copy_for_communities").setAttribute("checked", "true");
  parent = this.parentElement;
  try { parent.querySelector(".copy_for_profile").removeAttribute("checked") } catch { null };
  current_block = parent.nextElementSibling;
  current_block.querySelector(".collector").innerHTML = "";

  create_fullscreen("/users/load/communities/?type=" + this.getAttribute("data-type"), "worker_fullscreen")
});

on('#ajax', 'click', '#repost_for_wall', function() {
  this.querySelector("#repost_radio_wall").setAttribute("checked", "true");
  parent = this.parentElement;
  parent.querySelector("#repost_radio_community").removeAttribute("checked");
  parent.querySelector("#repost_radio_message").removeAttribute("checked");
  current_block = parent.nextElementSibling;
  current_block.querySelector(".collector").innerHTML = "";

  form = parent.parentElement.parentElement.parentElement.parentElement.parentElement;
  copy_case = form.querySelector("#toggle_case_item_copy");
  if (copy_case && copy_case.classList.contains("underline")) {
    url = "/users/load/post_lists/?type=" + form.querySelector(".item_type").value
  } else {
    url = "/users/load/post_lists/"
  };
  create_fullscreen(url, "worker_fullscreen")
});
on('#ajax', 'click', '#u_repost_for_community', function() {
  this.querySelector("#repost_radio_community").setAttribute("checked", "true");
  parent = this.parentElement;
  parent.querySelector("#repost_radio_wall").removeAttribute("checked");
  parent.querySelector("#repost_radio_message").removeAttribute("checked");
  current_block = parent.nextElementSibling;
  current_block.querySelector(".collector").innerHTML = "";

  form = parent.parentElement.parentElement.parentElement.parentElement.parentElement;
  copy_case = form.querySelector("#toggle_case_item_copy");
  if (copy_case && copy_case.classList.contains("underline")) {
    url = "/users/load/communities_post_lists/?type=" + form.querySelector(".item_type").value
  } else {
    url = "/users/load/communities_post_lists/"
  };
  create_fullscreen(url, "worker_fullscreen");
});
on('#ajax', 'click', '#repost_for_message', function() {
  this.querySelector("#repost_radio_message").setAttribute("checked", "true");
  parent = this.parentElement;
  parent.querySelector("#repost_radio_wall").removeAttribute("checked");
  parent.querySelector("#repost_radio_community").removeAttribute("checked");
  current_block = parent.nextElementSibling;
  current_block.querySelector(".collector").innerHTML = "";
  create_fullscreen("/users/load/chat_items/", "worker_fullscreen");
});

on('#ajax', 'click', '.u_post_edit', function() {
  block = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  if (block.querySelector(".post_edit_form")) {
    return
  } else {
    clear_attach_block();
    div = document.createElement("div");
    block.append(div);
    block.querySelector(".text_support") ? block.querySelector(".text_support").style.display = "none" : null;
    block.querySelector(".attach_container") ? block.querySelector(".attach_container").style.display = "none" : null;
    block.querySelector(".card-footer").style.display = "none";

    list_load(div, "/posts/user_progs/edit_post/" + block.getAttribute("data-pk") + "/");
  }
});

on('#ajax', 'click', '.u_article_detail', function() {
  uuid = this.parentElement.getAttribute("data-uuid");
  document.body.querySelector(".pk_saver") ? pk = document.body.querySelector(".pk_saver").getAttribute('data-pk') : pk = this.parentElement.getAttribute('data-pk');
  create_fullscreen("/article/detail/" + pk + "/" + uuid + "/", "item_fullscreen");
});

on('#ajax', 'click', '.u_load_comment_photo', function() {
  this.classList.add("current_file_dropdown");
  document.body.querySelector(".attach_block") ? (attach_block = document.body.querySelector(".attach_block"), attach_block.innerHTML = "", attach_block.classList.remove("attach_block")) : null;
  create_fullscreen('/users/load/u_img_comment_load/', "item_fullscreen");
});
on('#ajax', 'click', '.u_load_comment_video', function() {
  this.classList.add("current_file_dropdown");
  clear_attach_block();
  create_fullscreen('/users/load/u_video_load/', "item_fullscreen");
});
on('#ajax', 'click', '.u_load_comment_music', function() {
  this.classList.add("current_file_dropdown");
  clear_attach_block();
  create_fullscreen('/users/load/u_music_load/', "item_fullscreen");
});
on('#ajax', 'click', '.u_load_comment_doc', function() {
  this.classList.add("current_file_dropdown");
  clear_attach_block();
  create_fullscreen('/users/load/u_doc_load/', "item_fullscreen");
});
on('#ajax', 'click', '.u_load_comment_good', function() {
  this.classList.add("current_file_dropdown");
  clear_attach_block();
  create_fullscreen('/users/load/u_good_load/', "item_fullscreen");
});
on('#ajax', 'click', '.u_load_comment_article', function() {
  this.classList.add("current_file_dropdown");
  clear_attach_block();
  create_fullscreen('/users/load/u_article_load/', "item_fullscreen");
});

on('#ajax', 'click', '.u_select_photo', function() {
  this.parentElement.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  this.parentElement.classList.remove("show");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_img_load/', "item_fullscreen");
});
on('#ajax', 'click', '.u_select_survey', function() {
  this.parentElement.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  this.parentElement.classList.remove("show");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_survey_load/', "item_fullscreen");
});

on('#ajax', 'click', '.u_select_video', function() {
  this.parentElement.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  this.parentElement.classList.remove("show");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_video_load/', "item_fullscreen");
});
on('#ajax', 'click', '.u_select_music', function() {
  this.parentElement.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  this.parentElement.classList.remove("show");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_music_load/', "item_fullscreen");
});
on('#ajax', 'click', '.u_select_doc', function() {
  this.parentElement.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  this.parentElement.classList.remove("show");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_doc_load/', "item_fullscreen");
});
on('#ajax', 'click', '.u_select_good', function() {
  this.parentElement.classList.remove("show");
  this.parentElement.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_good_load/', "item_fullscreen");
});
on('#ajax', 'click', '.u_select_article', function() {
  this.parentElement.parentElement.parentElement.parentElement.previousElementSibling.classList.add("attach_block");
  this.parentElement.classList.remove("show");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_article_load/', "item_fullscreen");
});

on('#ajax', 'click', '.m_select_photo', function() {
  this.parentElement.classList.remove("show");
  this.parentElement.parentElement.parentElement.parentElement.previousElementSibling.classList.add("message_attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_img_message_load/', "item_fullscreen");
});
on('#ajax', 'click', '.m_select_video', function() {
  this.parentElement.classList.remove("show");
  this.parentElement.parentElement.parentElement.parentElement.previousElementSibling.classList.add("message_attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_video_load/', "item_fullscreen");
});
on('#ajax', 'click', '.m_select_music', function() {
  this.parentElement.classList.remove("show");
  this.parentElement.parentElement.parentElement.parentElement.previousElementSibling.classList.add("message_attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_music_load/', "item_fullscreen");
});
on('#ajax', 'click', '.m_select_doc', function() {
  this.parentElement.classList.remove("show");
  this.parentElement.parentElement.parentElement.parentElement.previousElementSibling.classList.add("message_attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_doc_load/', "item_fullscreen");
});
on('#ajax', 'click', '.m_select_good', function() {
  this.parentElement.classList.remove("show");
  this.parentElement.parentElement.parentElement.parentElement.previousElementSibling.classList.add("message_attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_good_load/', "item_fullscreen");
});
on('#ajax', 'click', '.m_select_article', function() {
  this.parentElement.classList.remove("show");
  this.parentElement.parentElement.parentElement.parentElement.previousElementSibling.classList.add("message_attach_block");
  clear_comment_dropdown();
  create_fullscreen('/users/load/u_article_load/', "item_fullscreen");
});

on('#ajax', 'click', '.delete_thumb', function(e) {
  e.preventDefault();
  this.nextElementSibling.remove();
  block = document.createElement("div");
  this.parentElement.innerHTML = "<h4>Изображение</h4><i>(обязательно)</i>";
  this.remove();
});
