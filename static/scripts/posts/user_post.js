on('body', 'click', '.comment_delete', function() {
  saver = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  type = this.parentElement.getAttribute("data-type");
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/users/progs/delete_comment/?type=" + type, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    div = document.createElement("div");
    div.classList.add("col-12");
    div.style.padding = "10px";
    div.style.display = "block";
    div.innerHTML = "Комментарий удалён. <span class='comment_recover pointer underline' data-type='" + type + "'>Восстановить</span>";
    saver.style.display = "none"; saver.parentElement.insertBefore(div, saver)
  }};
  link.send( );
});
on('body', 'click', '.comment_recover', function() {
  type = this.getAttribute("data-type");
  block = this.parentElement; next = block.nextElementSibling;
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/users/progs/recover_comment/?type=" + type, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    block.remove();
    next.style.display = "block";
  }};
  link.send();
});


on('#ajax', 'change', '.create_video_hide_file', function() {
  form = this.parentElement.parentElement.parentElement;
  pk = form.getAttribute("data-pk");

  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );

  link.open( 'POST', "/video/add_video/" + pk + "/", true )
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
  }};
  link.upload.onprogress = function(event) {
    count = event.loaded / event.total * 100;
    document.body.querySelector(".create_header").innerHTML = 'Загружено ' + Math.round(count) + '%';
  };
  link.upload.onload = function() {
    document.body.querySelector(".create_header").innerHTML = "Видеозапись загружена!"
  }
  link.send(form_data);
});

on('body', 'change', '.case_all_input', function() {
  _this = this, case_video = false, id_video_upload_start = false, is_video_edit_window_loaded = true;
  if (this.classList.contains("add_photos_in_list")) {
    url = "/gallery/add_photos_in_list/"
  } else if (this.classList.contains("add_tracks_in_list")) {
    url = "/music/add_tracks_in_list/"
  } else if (this.classList.contains("add_docs_in_list")) {
    url = "/docs/add_docs_in_list/"
  } else if (this.classList.contains("add_video_in_list")) {
    if (_this.files[0].type != "video/mp4") {
      toast_info("Пока работаем только с mp4");
      return
    };
    url = "/video/add_video_in_list/";
    case_video = true;
  };

  form = this.parentElement.parentElement
  if (form.getAttribute("data-pk")) {
    url = url + form.getAttribute("data-pk") + "/"
  };
  form_data = new FormData(form);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );

  link_.open( 'POST', url, true )
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
      if (case_video) {
        jsonResponse = JSON.parse(link_.responseText);
        document.body.querySelector("#upload_video_pk").setAttribute("value", jsonResponse.pk)
      }
      else { 
        response = document.createElement("span");
        response.innerHTML = link_.responseText;
        lenta = form.parentElement.parentElement.parentElement.querySelector(".is_paginate");
        lenta.insertAdjacentHTML('afterBegin', response.innerHTML);
        lenta.querySelector(".items_empty") ? document.body.querySelector(".items_empty").style.display = "none" : null
      }
  }};
  link_.upload.onprogress = function(event) {
    if (case_video) {
      if (!id_video_upload_start) {
        close_work_fullscreen();
        id_video_upload_start = true;
        create_fullscreen("/video/edit_video/", "worker_fullscreen");
        //fullscreen_resize()
      };
      if (is_video_edit_window_loaded) {
        try {
          title = document.body.querySelector("#id_title");
          title.value = _this.files[0].name;
          title.select();
          is_video_edit_window_loaded = false
        } catch { null }
      }
    };
    count = event.loaded / event.total * 100;
    try {
      document.body.querySelector("#onload_info").innerHTML = 'Загружено ' + Math.round(count) + '%'
    } catch { null }
  };
  link_.upload.onload = function() {
    try {
      info = document.body.querySelector("#onload_info");
      if (case_video) {
        info.innerHTML = "Видео загружено!";
        document.body.querySelector("#edit_video_btn").classList.remove("hidden")
      } else { info.innerHTML = "" }
    } catch { null }
  };
  link_.send(form_data);
});

on('body', 'click', '.photo_attach_list_remove', function() {
  block = this.parentElement.parentElement;
  if (block.parentElement.classList.contains("attach_block")){
    remove_file_attach(), is_full_attach()
  } else if (block.classList.contains("comment_attach_block")){
    remove_file_dropdown(); is_full_dropdown()
  } else if (block.classList.contains("message_attach_block")){
    remove_file_message_attach(); is_full_message_attach()
  }
  block.remove();
});

on('#ajax', 'click', '#u_add_article', function() {
  pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");
  create_fullscreen("/article/u_article_window/" + pk + "/", "item_fullscreen");
});

on('#ajax', 'click', '#u_add_post_btn', function() {
  form_post = this.parentElement.parentElement.parentElement.parentElement;
  text_val = form_post.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;
  if (_text.replace(/<(?!img)\/?[a-z][^>]*(>|$)/gi, "").trim() == "" && form_post.querySelector(".files_0")) {
    toast_error("Напишите или прикрепите что-нибудь"); return
  };

  $input = document.createElement("input");
  $input.setAttribute("name", "text");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = _text;
  form_post.append($input);
  form_data = new FormData(form_post);

  lenta_load = form_post.parentElement.nextElementSibling.nextElementSibling;
  pk = form_post.parentElement.parentElement.getAttribute("data-uuid");

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/posts/user_progs/add_post/" + pk + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    clear_attach_block();

    elem = link_.responseText;
    new_post = document.createElement("span");
    new_post.innerHTML = elem;
    drops = form_post.querySelectorAll(".dropdown-menu");
    form_post.querySelector(".input_text").remove();
    form_post.querySelector(".smile_supported").innerHTML = "";
    for (var i = 0; i < drops.length; i++){drops[i].classList.remove("show")}
    lenta_load.insertAdjacentHTML('afterBegin', new_post.innerHTML);
    toast_info('Запись опубликована');
    lenta_load.querySelector(".items_empty") ? lenta_load.querySelector(".items_empty").style.display = "none" : null;
    main_container = document.body.querySelector(".main-container");
    add_list_in_all_stat("created_user_post",new_post.querySelector(".pag").getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
  } else {
        new_post = document.createElement("span");
        new_post.innerHTML = link_.responseText;
        if (new_post.querySelector(".exception_value")){
          text = new_post.querySelector(".exception_value").innerHTML;
          toast_info(text)
        }
    }
  };

  link_.send(form_data);
});

on('body', 'click', '.comment_edit', function() {
  _this = this;
  clear_comment_dropdown();

  type = _this.parentElement.getAttribute("data-type");
  _this.parentElement.style.display = "none";
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/users/progs/edit_comment/?type=" + type, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link.responseText;
    response = document.createElement("span");
    response.innerHTML = elem;
    parent = _this.parentElement.parentElement.parentElement;

    parent.parentElement.querySelector(".comment_text").style.display = "none";
    parent.parentElement.querySelector(".attach_container") ? parent.parentElement.querySelector(".attach_container").style.display = "none" : null;
    parent.append(response);
  }};
  link.send( null );
});

on('body', 'click', '.comment_edit_btn', function() {
  form = this.parentElement.parentElement.parentElement
  _text = form.querySelector(".smile_supported").innerHTML;
  if (_text.replace(/<[^>]*(>|$)|&nbsp;|&zwnj;|&raquo;|&laquo;|&gt;/g,'').trim() == "" && !form.querySelector(".img_block").firstChild){
    toast_error("Напишите или прикрепите что-нибудь");
    form.querySelector(".text-comment").style.border = "1px #FF0000 solid";
    form.querySelector(".dropdown").style.border = "1px #FF0000 solid";
    return
  };

  span_form = form.parentElement;
  block = span_form.parentElement.parentElement.parentElement;
  $input = document.createElement("input");
  $input.setAttribute("name", "text");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = form.querySelector(".smile_supported").innerHTML;
  form.append($input);
  form_comment = new FormData(form);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link_.open('POST', "/users/progs/edit_comment/", true);
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem = link_.responseText;
          new_post = document.createElement("span");
          new_post.innerHTML = elem;
          block.querySelector(".media-body").innerHTML = new_post.querySelector(".media-body").innerHTML;
          toast_success("Комментарий изменен");
      }
  };
  link_.send(form_comment)
});

/*!
   item post scripts for user
  */
on('#ajax', 'click', '.u_post_remove', function() {
  item = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  pk = item.getAttribute("data-pk");
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/posts/user_progs/delete/" + pk + "/", true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    p = document.createElement("div");
    p.classList.add("card", "mb-3");
    p.style.padding = "20px";
    p.innerHTML = "<span class='u_post_restore pointer' data-pk='" + pk + "'>Запись удалена. <span class='underline'>Восстановить</span></span>";
    !document.querySelector(".post_detail") ? (item.parentElement.insertBefore(p, item), item.style.display = "none")
    : (document.querySelector(".item_fullscreen").style.display = "none",
    block = document.body.querySelector(".post_stream"),
    item = block.querySelector( '[data-pk=' + '"' + pk + '"' + ']' ),
    item.style.display = "none",
    p.style.display =  "block",
    item.parentElement.insertBefore(p, item));
    main_container = document.body.querySelector(".main-container");
    add_list_in_all_stat("deleted_user_post",pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
  }};

  link.send( );
});

on('#ajax', 'click', '.u_post_restore', function() {
  item = this.parentElement.nextElementSibling;
  pk = this.getAttribute("data-pk");
  block = this.parentElement;
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/posts/user_progs/restore/" + pk + "/", true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    block.remove();
    item.style.display = "block";
    main_container = document.body.querySelector(".main-container");
    add_list_in_all_stat("restored_user_post",pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
  }};
  link.send();
});

on('#ajax', 'click', '.u_post_fixed', function() {
  item = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  pk = item.getAttribute("data-pk");
  send_change(this, "/posts/user_progs/fixed/", "u_post_unfixed", "Открепить");
  main_container = document.body.querySelector(".main-container");
  add_list_in_all_stat("fixed_user_post",pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});
on('#ajax', 'click', '.u_post_unfixed', function() {
  item = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  pk = item.getAttribute("data-pk");
  send_change(this, "/posts/user_progs/unfixed/", "u_post_fixed", "Закрепить");
  main_container = document.body.querySelector(".main-container");
  add_list_in_all_stat("unfixed_user_post",pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});

on('#ajax', 'click', '.u_post_off_comment', function() {
  send_change(this, "/posts/user_progs/off_comment/", "u_post_on_comment", "Вкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_post_comments") ? post.querySelector(".load_post_comments").style.display = "none"
  : post.querySelector(".u_news_item_comments").style.display = "none";
  main_container = document.body.querySelector(".main-container");
  add_list_in_all_stat("off_comment_user_post",post.getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
})
on('#ajax', 'click', '.u_post_on_comment', function() {
  send_change(this, "/posts/user_progs/on_comment/", "u_post_off_comment", "Выкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_post_comments") ? post.querySelector(".load_post_comments").style.display = "unset"
  : post.querySelector(".u_news_item_comments").style.display = "unset";
  main_container = document.body.querySelector(".main-container");
  add_list_in_all_stat("on_comment_user_post",post.getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});

on('#ajax', 'click', '.u_post_off_votes', function() {
  send_change(this, "/posts/user_progs/off_votes/", "u_post_on_votes", "Вкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "none";
  post.querySelector(".dislike").style.display = "none";
  main_container = document.body.querySelector(".main-container");
  add_list_in_all_stat("off_votes_user_post",post.getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});
on('#ajax', 'click', '.u_post_on_votes', function() {
  send_change(this, "/posts/user_progs/on_votes/", "u_post_off_votes", "Выкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "unset";
  post.querySelector(".dislike").style.display = "unset";
  main_container = document.body.querySelector(".main-container");
  add_list_in_all_stat("on_votes_user_post",post.getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});

on('body', 'click', '.like_item', function() {
  _this = this;
  item = _this.parentElement;
  send_like(item, "/users/progs/like_item/?type=" + item.getAttribute("data-type"));
  like_reload(this.nextElementSibling, this.nextElementSibling.nextElementSibling.nextElementSibling, "item_likes")
  main_container = document.body.querySelector(".main-container");
  //add_list_in_all_stat("dislike_user_post_comment",comment_pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});
on('body', 'click', '.dislike_item', function() {
  _this = this;
  item = _this.parentElement;
  send_dislike(item, "/users/progs/dislike_item/?type=" + item.getAttribute("data-type"));
  dislike_reload(this.previousElementSibling, this.nextElementSibling, "item_dislikes");

  //main_container = document.body.querySelector(".main-container");
  //add_list_in_all_stat("dislike_user_post_comment",comment_pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});

on('body', 'click', '.delete_list', function() {
  _this = this;
  _this.removeAttribute('tooltip');
  parent = _this.parentElement;
  type = parent.getAttribute('data-type');
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'GET', "/users/progs/delete_list/?type=" + type , true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    hide_icons = parent.parentElement.querySelectorAll(".hide_delete");
    for (var i = 0; i < hide_icons.length; i++){
      hide_icons[i].style.display = "none";
    };
    parent.parentElement.querySelector(".second_list_name").innerHTML = "";
    list = document.body.querySelector( '[data-pk=' + '"' + type.slice(3) + '"' + ']' );
    list.querySelector('.list_name').innerHTML = "Список удален";
    _this.classList.replace("delete_list", "recover_list");
    _this.innerHTML = "Восстановить список";
    //main_container = document.body.querySelector(".main-container");
    //add_list_in_all_stat(stat_class,type.slice(3),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
  }}
  link_.send();
});
on('body', 'click', '.recover_list', function() {
  _this = this;
  _this.setAttribute('tooltip', 'Удалить список');
  parent = _this.parentElement;
  type = parent.getAttribute('data-type');
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'GET', "/users/progs/recover_list/?type=" + type, true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    hide_icons = parent.parentElement.querySelectorAll(".hide_delete");
    for (var i = 0; i < hide_icons.length; i++){
      hide_icons[i].style.display = "unset";
    };
    second_list = document.body.querySelector('.second_list_name');
    name = second_list.getAttribute("data-name");
    second_list.innerHTML = name;
    list = document.body.querySelector( '[data-pk=' + '"' + type.slice(3) + '"' + ']' );
    list.querySelector('.list_name').innerHTML = name;
    _this.classList.replace("recover_list", "delete_list");
    _this.innerHTML = '<svg class="svg_info" fill="currentColor" viewBox="0 0 24 24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M16.5,10V9h-2v1H12v1.5h1v4c0,0.83,0.67,1.5,1.5,1.5h2c0.83,0,1.5-0.67,1.5-1.5v-4h1V10H16.5z M16.5,15.5h-2v-4h2V15.5z M20,6h-8l-2-2H4C2.89,4,2.01,4.89,2.01,6L2,18c0,1.11,0.89,2,2,2h16c1.11,0,2-0.89,2-2V8C22,6.89,21.11,6,20,6z M20,18H4V6h5.17 l2,2H20V18z"/></g></svg>';
    //main_container = document.body.querySelector(".main-container");
    //add_list_in_all_stat(stat_class,type.slice(3),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
  }}
  link_.send();
});

on('body', 'click', '.like2', function() {
  _this = this;
  item = _this.parentElement;
  send_like(item, "/users/progs/like_comment/?type=" + item.getAttribute("data-type"));
  like_reload(this.nextElementSibling, this.nextElementSibling.nextElementSibling.nextElementSibling, "comment_likes")
  main_container = document.body.querySelector(".main-container");
  //add_list_in_all_stat("dislike_user_post_comment",comment_pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});
on('body', 'click', '.dislike2', function() {
  _this = this;
  item = _this.parentElement;
  send_dislike(item, "/users/progs/dislike_comment/?type=" + item.getAttribute("data-type"));
  dislike_reload(this.previousElementSibling, this.nextElementSibling, "comment_dislikes");

  main_container = document.body.querySelector(".main-container");
  //add_list_in_all_stat("dislike_user_post_comment",comment_pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));
});

on('#ajax', 'change', '#u_photo_post_attach', function() {
  form = this.parentElement;
  form_data = new FormData(form);
  input = form.querySelector(".upload_for_post_attach")
  if (input.files.length > 10) {
      toast_error("Не больше 10 фотографий");
      return;
  }
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/gallery/user_progs/add_attach_photo/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    response = document.createElement("span");
    response.innerHTML = elem;
    photo_list = response.querySelectorAll(".pag");
    if (document.body.querySelector(".attach_block")){
      block = document.body.querySelector(".attach_block");
      photo_post_upload_attach(photo_list, block);
    } else if (document.body.querySelector(".message_attach_block")){
      block = document.body.querySelector(".message_attach_block");
      photo_message_upload_attach(photo_list, block);
    }
    }
    close_work_fullscreen();
  }
  link_.send(form_data);
});

on('#ajax', 'change', '#u_photo_post_comment_attach', function() {
  form = document.body.querySelector("#add_comment_photos");
  form_data = new FormData(form);
  input = form.querySelector("#u_photo_post_comment_attach")
  if (input.files.length > 2) {
      toast_error("Не больше 2 фотографий");
      return;
  }
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/gallery/user_progs/add_attach_photo/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    response = document.createElement("span");
    response.innerHTML = elem;
    photo_list = response.querySelectorAll(".pag");
    photo_comment_upload_attach(photo_list, document.body.querySelector(".current_file_dropdown").parentElement.parentElement, photo_list.length);
    }
    close_work_fullscreen();
  }
  link_.send(form_data);
});

on('#ajax', 'click', '.photo_load_several', function() {
  previous = this.previousElementSibling
  _this = previous.querySelector("img");
  photo_pk = previous.getAttribute('photo-pk');
  user_pk = previous.getAttribute('data-pk');
  src = _this.parentElement.getAttribute("data-href");
  if (document.body.querySelector(".current_file_dropdown")){
    check_photo_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, photo_pk) ? null : (photo_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, photo_pk, user_pk, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".attach_block")){
    check_photo_in_block(document.body.querySelector(".attach_block"), _this, photo_pk) ? null : (photo_post_attach(document.body.querySelector(".attach_block"), photo_pk, user_pk, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".message_attach_block")){
    check_photo_in_block(document.body.querySelector(".message_attach_block"), _this, photo_pk) ? null : (photo_message_attach(document.body.querySelector(".message_attach_block"), photo_pk, user_pk, src), this.classList.add("active_svg"), show_message_form_send_btn())
  }
});

on('#ajax', 'click', '.photo_load_one', function() {
  _this = this;
  photo_pk = _this.parentElement.getAttribute('photo-pk');
  user_pk = _this.parentElement.getAttribute('data-pk');
  src = _this.parentElement.getAttribute("data-href");
  if (document.body.querySelector(".attach_block")){
    check_photo_in_block(document.body.querySelector(".attach_block"), _this, photo_pk) ? null : (photo_post_attach(document.body.querySelector(".attach_block"), photo_pk, user_pk, src), close_work_fullscreen())
  }
  else if (document.body.querySelector(".current_file_dropdown")){
    check_photo_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, photo_pk) ? null : (photo_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, photo_pk, user_pk, src), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_photo_in_block(document.body.querySelector(".message_attach_block"), _this, photo_pk) ? null : (close_work_fullscreen(), photo_message_attach(document.body.querySelector(".message_attach_block"), photo_pk, user_pk, src))
  }
});

on('#ajax', 'click', '.u_create_video_attach_btn', function() {
  form_data = new FormData(document.querySelector("#create_video_form"));
  user_pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/video/progs/create_video_attach/" + user_pk + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem_ = document.createElement('div');
    elem_.innerHTML = link_.responseText;

    dropdown = document.body.querySelector(".current_file_dropdown").parentElement.parentElement;
    video_comment_attach(elem_.querySelector("img"), dropdown);

    close_work_fullscreen();
  }};

  link_.send(form_data);
});

on('#ajax', 'click', '#create_repost_btn', function() {
  form_post = this.parentElement.parentElement;
  collector = form_post.querySelector(".collector");
  if (!collector.innerHTML) {
    collector.innerHTML = '<div class="response_text">⇠ <br>Выберите списки записей или получателей</div>';
    return
  }
  text_val = form_post.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;

  $input = document.createElement("input");
  $input.setAttribute("name", "text");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = _text;
  form_post.append($input);

  form_data = new FormData(form_post);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/users/progs/create_repost/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    close_work_fullscreen();
    toast_info("Репост сделан!")
  }};

  link_.send(form_data);
});

on('#ajax', 'click', '#create_copy_btn', function() {
  form_post = this.parentElement.parentElement;
  collector = form_post.querySelector(".collector");
  if (!form_post.querySelector(".is_list") && !collector.innerHTML) {
    collector.innerHTML = '<div class="response_text">⇠ <br>Выберите списки</div>';
    return
  }
  else if (form_post.querySelector(".is_list") && form_post.querySelector(".copy_for_communities").checked && !collector.innerHTML) {
    collector.innerHTML = '<div class="response_text">⇠ <br>Выберите сообщества</div>';
    return
  }
  else if (form_post.querySelector(".is_list") && !form_post.querySelector(".copy_for_communities").checked && !form_post.querySelector(".copy_for_profile").checked) {
    collector.innerHTML = '<div class="response_text">Выберите, куда копировать объект</div>';
    return
  };

  form_data = new FormData(form_post);
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/users/progs/create_copy/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    close_work_fullscreen();
    toast_info("Объект копирован!")
  }};
  link_.send(form_data);
});

on('#ajax', 'click', '#create_list_btn', function() {
  form_post = this.parentElement.parentElement.parentElement;
  type = form_post.querySelector(".type").value;
  console.log(type.slice(0,3));
  if (!form_post.querySelector("#id_name").value){
    form_post.querySelector("#id_name").style.border = "1px #FF0000 solid";
    toast_error("Название - обязательное поле!");
    return
  } else { this.disabled = true };

  form_data = new FormData(form_post);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/users/progs/create_list/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    elem = link_.responseText;
    new_post = document.createElement("span");
    new_post.innerHTML = elem;

    if (type.slice(0,3) == "lpo") {
      post_stream = document.body.querySelector(".span_list_pk");
      post_stream.innerHTML = '';
      post_stream.innerHTML = '<div class="card mb-3 items_empty centered"><div class="card-body"><svg fill="currentColor" class="thumb_big svg_default" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0z"/><path fill="currentColor" d="M22 13h-8v-2h8v2zm0-6h-8v2h8V7zm-8 10h8v-2h-8v2zm-2-8v6c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V9c0-1.1.9-2 2-2h6c1.1 0 2 .9 2 2zm-1.5 6l-2.25-3-1.75 2.26-1.25-1.51L3.5 15h7z"/></svg></div><h6 style="margin: 20px;">Пока записей нет...</h6></div>';

      userpic = document.body.querySelector(".userpic");

      name = form_post.querySelector("#id_name").value;
      li = document.createElement("li");
      li.classList.add("date", "list", "active");
      new_pk = new_post.querySelector(".span_list_pk").getAttribute("list-pk");
      li.setAttribute("list-pk", new_pk);

      media = document.createElement("div");
      media.classList.add("media");

      media_body = document.createElement("div");
      media_body.classList.add("media-body");

      h6 = document.createElement("h6");
      h6.classList.add("my-0", "mt-1");
      h6.innerHTML = '<span class="list_name">' + name + '</span> (<span class="handle">0</span>)';

      figure = document.createElement("figure");

      if (userpic.querySelector("img")) {
        a = document.createElement("a");
        a.classList.add("ajax");
        a.setAttribute("href", userpic.getAttribute("data-pk"));
        img = document.createElement("img");
        img.setAttribute("src", userpic.querySelector("img").getAttribute("src"));
        img.style.borderRadius = "30px";
        img.style.width = "30px";
        figure.append(img);
        a.append(figure);
      } else {
        a = document.createElement("span");
        a.innerHTML = '<svg fill="currentColor" class="svg_default svg_default_30" viewBox="0 0 24 24"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"></path><path d="M0 0h24v24H0z" fill="none"></path></svg>'
        h6.classList.add("ml-2");
      };

      media_body.append(h6);
      media.append(a);
      media.append(media_body);
      li.append(media);
      document.body.querySelector(".date-list").prepend(li);
    }
    else {

      ajax = new_post.querySelector("#reload_block");
      rtr = document.getElementById('ajax');
      rtr.innerHTML = ajax.innerHTML;
      window.scrollTo(0,0);
      document.title = new_post.querySelector('title').innerHTML;
      window.history.pushState({route: new_post.location.href}, "network", new_post.location.href);
    };
    close_work_fullscreen();
  }};

  link_.send(form_data);
});

on('#ajax', 'click', '#edit_list_btn', function() {
  form = this.parentElement.parentElement.parentElement;
  type = form.querySelector(".type").value;
  form_data = new FormData(form);
  if (!form.querySelector("#id_name").value){
    form.querySelector("#id_name").style.border = "1px #FF0000 solid";
    toast_error("Название - обязательное поле!");
    return
  } else { this.disabled = true }
  pk = form.getAttribute("data-pk")

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/users/progs/edit_list/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    close_work_fullscreen();
    name = form.querySelector('#id_name').value;

    if (type.slice(0,3) == "lpo") {
      title = document.body.querySelector( '[list-pk=' + '"' + pk + '"' + ']' );
      title.querySelector(".list_name").innerHTML = name }
    else {
      list = document.body.querySelector( '[data-pk=' + '"' + pk + '"' + ']' );
      list.querySelector('.list_name') ? list.querySelector('.list_name').innerHTML = name : null;
      document.body.querySelector('.second_list_name').innerHTML = name;
    };
    toast_success("Список изменен");
    //main_container = document.body.querySelector(".main-container");
    //add_list_in_all_stat(stat_class,pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
  }}
  link_.send(form_data);
});

on('#ajax', 'click', '#create_claim_btn', function() {
  form_post = this.parentElement.parentElement;

  text_val = form_post.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;

  $input = document.createElement("input");
  $input.setAttribute("name", "description");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = _text;
  form_post.append($input);

  form_data = new FormData(form_post);

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/users/progs/create_claim/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    close_work_fullscreen();
    toast_info("Жалоба отправлена!")
  }};

  link_.send(form_data);
});

on('#ajax', 'click', '.remove_list_in_user_collections', function() {
  _this = this;
  a = "u" + _this.getAttribute("data-pk");
  form = _this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  input = form.querySelector(".item_type").value
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'GET', "/users/progs/uncopy_user_list/?type=" + form.querySelector(".item_type").value, true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    close_work_fullscreen()
  }};

  link_.send();
});
on('#ajax', 'click', '.remove_list_in_community_collections', function() {
  _this = this;
  pk = _this.getAttribute("data-pk");
  type = _this.getAttribute("data-type");
  block = _this.parentElement.parentElement.parentElement;
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'GET', "/users/progs/uncopy_community_list/" + pk + "/?type=" + type, true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( link_.readyState == 4 && link_.status == 200 ) {
    parent = _this.parentElement;
    parent.innerHTML = "";
    parent.innerHTML = "Сообщество";
    block.classList.add("communities_toggle", "pointer");
  }};

  link_.send();
});

on('#ajax', 'click', '.video_load_one', function() {
  _this = this;
  pk = _this.getAttribute('video-pk');
  counter = _this.getAttribute('video-counter');
  src = _this.getAttribute('src');
  if (document.body.querySelector(".current_file_dropdown")){
    check_video_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (video_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, pk, counter, src), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_video_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (video_post_attach(document.body.querySelector(".attach_block"), pk, counter, src), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_video_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (video_message_attach(document.body.querySelector(".message_attach_block"), pk, counter, src), close_work_fullscreen(), show_message_form_send_btn())
  }
});
on('#ajax', 'click', '.video_load_several', function() {
  previous = this.previousElementSibling
  _this = previous.querySelector("img");
  pk = _this.getAttribute('video-pk');
  counter = _this.getAttribute('video-counter');
  src = _this.getAttribute('src');
  if (document.body.querySelector(".current_file_dropdown")){
    check_video_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (video_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, pk, counter, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".attach_block")){
    check_video_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (video_post_attach(document.body.querySelector(".attach_block"), pk, counter, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".message_attach_block")){
    check_video_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (video_message_attach(document.body.querySelector(".message_attach_block"), pk, counter, src), this.classList.add("active_svg"), show_message_form_send_btn())
  }
});
on('body', 'click', '.video_attach_list', function() {
  _this = this;
  name = _this.parentElement.querySelector(".list_name").innerHTML;
  pk = _this.getAttribute('data-pk');
  count = _this.parentElement.querySelector(".count").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_video_list_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (video_list_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_video_list_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (video_list_post_attach(document.body.querySelector(".attach_block"), name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_video_list_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (video_list_message_attach(document.body.querySelector(".message_attach_block"), name, pk, count), close_work_fullscreen())
  }
});

on('#ajax', 'click', '.music_load_one', function() {
  _this = this;
  pk = _this.getAttribute('music-pk');
  counter = _this.getAttribute('music-counter');
  _this.querySelector("img") ? src = _this.querySelector("img").getAttribute('src') : src = '/static/images/no_track_img.jpg'
  if (document.body.querySelector(".current_file_dropdown")){
    check_music_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, counter) ? null : (music_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, pk, counter, src), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_music_in_block(document.body.querySelector(".attach_block"), _this, counter) ? null : (music_post_attach(document.body.querySelector(".attach_block"), pk, counter, src), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_music_in_block(document.body.querySelector(".message_attach_block"), _this, counter) ? null : (music_message_attach(document.body.querySelector(".message_attach_block"), pk, counter, src), close_work_fullscreen(), show_message_form_send_btn())
  }
  close_work_fullscreen();
});
on('#ajax', 'click', '.music_load_several', function() {
  _this = this.previousElementSibling
  pk = _this.getAttribute('music-pk');
  counter = _this.getAttribute('music-counter');
  _this.querySelector("img") ? src = _this.querySelector("img").getAttribute('src') : src = '/static/images/no_track_img.jpg'
  if (document.body.querySelector(".current_file_dropdown")){
    check_music_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, counter) ? null : (music_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, pk, counter, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".attach_block")){
    check_music_in_block(document.body.querySelector(".attach_block"), _this, counter) ? null : (music_post_attach(document.body.querySelector(".attach_block"), pk, counter, src), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".message_attach_block")){
    check_music_in_block(document.body.querySelector(".message_attach_block"), _this, counter) ? null : (music_message_attach(document.body.querySelector(".message_attach_block"), pk, counter, src), this.classList.add("active_svg"), show_message_form_send_btn())
  }
});
on('body', 'click', '.music_attach_list', function() {
  _this = this;
  name = _this.parentElement.querySelector(".list_name").innerHTML;
  pk = _this.getAttribute('data-pk');
  count = _this.parentElement.querySelector(".count").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_playlist_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (playlist_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_playlist_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (playlist_post_attach(document.body.querySelector(".attach_block"), name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_playlist_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (playlist_message_attach(document.body.querySelector(".message_attach_block"), name, pk, count), close_work_fullscreen())
  }
});

on('#ajax', 'click', '.doc_load_several', function() {
  _this = this.previousElementSibling;
  pk = _this.getAttribute('data-pk');
  media_block = _this.querySelector(".media-body")
  if (document.body.querySelector(".current_file_dropdown")){
    check_doc_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (doc_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, media_block, pk), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".attach_block")){
    check_doc_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (doc_post_attach(document.body.querySelector(".attach_block"), media_block, pk), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".message_attach_block")){
    check_doc_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (doc_message_attach(document.body.querySelector(".message_attach_block"), media_block, pk), this.classList.add("active_svg"), show_message_form_send_btn())
  }
});
on('body', 'click', '.doc_attach_list', function() {
  _this = this;
  name = _this.parentElement.querySelector(".list_name").innerHTML;
  pk = _this.getAttribute('data-pk');
  count = _this.parentElement.querySelector(".count").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_doc_list_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (doc_list_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_doc_list_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (doc_list_post_attach(document.body.querySelector(".attach_block"), name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_doc_list_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (doc_list_message_attach(document.body.querySelector(".message_attach_block"), name, pk, count), close_work_fullscreen())
  }
});

on('#ajax', 'click', '.survey_attach_remove', function() {
  block = this.parentElement.parentElement;
  block.remove();
  if (document.body.querySelector(".message_attach_block")){
    remove_file_message_attach()
  }
  else if (document.body.querySelector(".current_file_dropdown")){
    remove_file_dropdown()
  }
  else if (document.body.querySelector(".attach_block")){
    remove_file_attach()
  }
});

on('#ajax', 'click', '.good_load_one', function() {
  _this = this;
  data_pk = _this.getAttribute('good-pk');
  data_uuid = _this.getAttribute('good-uuid');
  src = _this.querySelector("img").getAttribute('src');
  title = _this.querySelector(".good_title").innerHTML;

  if (document.body.querySelector(".current_file_dropdown")){
    check_good_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, data_pk) ? null : (good_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, src, data_pk, data_uuid, title), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_good_in_block(document.body.querySelector(".attach_block"), _this, data_pk) ? null : (good_post_attach(document.body.querySelector(".attach_block"), src, data_pk, data_uuid, title), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_good_in_block(document.body.querySelector(".message_attach_block"), _this, data_pk) ? null : (good_message_attach(document.body.querySelector(".message_attach_block"), src, data_pk, data_uuid, title), close_work_fullscreen(), show_message_form_send_btn())
  }
});
on('#ajax', 'click', '.good_load_several', function() {
  _this = this.previousElementSibling;
  data_pk = _this.getAttribute('good-pk');
  data_uuid = _this.getAttribute('good-uuid');
  src = _this.querySelector("img").getAttribute('src');
  title = _this.querySelector(".good_title").innerHTML;

  if (document.body.querySelector(".current_file_dropdown")){
    check_good_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, data_pk) ? null : (good_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, src, data_pk, data_uuid, title), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".attach_block")){
    check_good_in_block(document.body.querySelector(".attach_block"), _this, data_pk) ? null : (good_post_attach(document.body.querySelector(".attach_block"), src, data_pk, data_uuid, title), this.classList.add("active_svg"))
  } else if (document.body.querySelector(".message_attach_block")){
    check_good_in_block(document.body.querySelector(".message_attach_block"), _this, data_pk) ? null : (good_message_attach(document.body.querySelector(".message_attach_block"), src, data_pk, data_uuid, title), this.classList.add("active_svg"), show_message_form_send_btn())
  }
});
on('body', 'click', '.good_attach_list', function() {
  _this = this;
  name = _this.parentElement.querySelector(".list_name").innerHTML;
  pk = _this.getAttribute('data-pk');
  count = _this.parentElement.querySelector(".count").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_good_list_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (good_list_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_good_list_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (good_list_post_attach(document.body.querySelector(".attach_block"), name, pk, count), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_good_list_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (good_list_message_attach(document.body.querySelector(".message_attach_block"), name, pk, count), close_work_fullscreen())
  }
});

on('#ajax', 'click', '.commmunty_load_one', function() {
  _this = this;
  block = _this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  commmunity_form_selected(_this, block.querySelector("#selected_message_target_items"))
});
on('#ajax', 'click', '.chat_item_load_one', function() {
  _this = this;
  block = _this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  chat_item_form_selected(_this, block.querySelector("#selected_message_target_items"))
});
on('#ajax', 'click', '.chat_friends_load_one', function() {
  _this = this;
  block = this.parentElement.parentElement.nextElementSibling;
  chat_item_form_selected(_this, block)
});

on('#ajax', 'click', '.attach_survey', function() {
  _this = this;
  pk = _this.getAttribute('data-pk');
  figure = _this.querySelector(".background-img");
  figure.querySelector("src") ? img_src = figure.querySelector("src").getAttribute("src") : img_src = null;
  container_html = _this.querySelector(".container").innerHTML;
  if (document.body.querySelector(".current_file_dropdown")){
    check_survey_in_block(document.body.querySelector(".current_file_dropdown").parentElement.parentElement.parentElement.parentElement.previousElementSibling, _this, pk) ? null : (survey_comment_attach(document.body.querySelector(".current_file_dropdown").parentElement.parentElement, pk, img_src, container_html), close_work_fullscreen())
  } else if (document.body.querySelector(".attach_block")){
    check_survey_in_block(document.body.querySelector(".attach_block"), _this, pk) ? null : (survey_post_attach(document.body.querySelector(".attach_block"), pk, img_src, container_html), close_work_fullscreen())
  } else if (document.body.querySelector(".message_attach_block")){
    check_survey_in_block(document.body.querySelector(".message_attach_block"), _this, pk) ? null : (survey_message_attach(document.body.querySelector(".message_attach_block"), pk, img_src, container_html), close_work_fullscreen())
  }
});
