on('#ajax', 'click', '#community_article_add', function() {
  pk = this.getAttribute('data-pk');
  create_fullscreen("/article/c_article_window/" + pk + "/", "item_fullscreen");
});

on('#ajax', 'click', '#c_add_post_btn', function() {
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
  link_.open( 'POST', "/posts/community_progs/add_post/" + pk + "/", true );
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
    add_list_in_all_stat("created_community_post",new_post.querySelector(".pag").getAttribute("data-pk"),main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"))
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

on('#ajax', 'click', '#c_edit_post_btn', function() {
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
  $input.value = form_post.querySelector(".smile_supported").innerHTML;
  form_post.append($input);
  form_data = new FormData(form_post);
  block = form_post.parentElement.parentElement;
  pk = block.getAttribute("data-pk");
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/posts/community_progs/edit_post/" + pk + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    clear_attach_block();
    elem = link_.responseText;
    new_post = document.createElement("span");
    new_post.innerHTML = elem;
    form_post.parentElement.remove();
    block.querySelector(".fullscreen") ? block.querySelector(".fullscreen").remove() : null;
    block.querySelector(".attach_container") ? block.querySelector(".attach_container").remove() : null;
    block.querySelector(".card-footer").remove()
    if (new_post.querySelector(".fullscreen")) {
      block.append(new_post.querySelector(".fullscreen"))
    }
    if (new_post.querySelector(".attach_container")) {
      block.append(new_post.querySelector(".attach_container"))
    };
    block.append(new_post.querySelector(".card-footer"));
    main_container = document.body.querySelector(".main-container");
    add_list_in_all_stat("edited_community_post",pk,main_container.getAttribute("data-type"),main_container.getAttribute("data-pk"));

  }};

  link_.send(form_data);
});

on('#ajax', 'click', '#c_add_offer_post', function() {
  form_post = document.body.querySelector("#admin_offer_post");
  form_data = new FormData(form_post);

  pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");

  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/posts/community_progs/add_offer_post/" + pk + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    form_post.querySelector('.id_text').value = "";
    clear_attach_block();
    drops = form_post.querySelectorAll(".dropdown-menu");
    for (var i = 0; i < drops.length; i++){drops[i].classList.remove("show")}
    elem = link_.responseText;
    document.body.querySelector(".user_draft_list") ? (toast_info("Запись предложена"),
                                                       value = document.body.querySelector(".user_draft_count").innerHTML,
                                                       value = value*1,
                                                       value += 1,
                                                       document.body.querySelector(".user_draft_count").innerHTML = value)
    : (document.body.querySelector(".draft_post_container").innerHTML = '<div class="card mt-3 user_draft_list"><div class="card-header"><a href="/communities/user_draft/' + pk + '/" class="ajax"><div class="media"><div class="media-body"><h4 class="content-color-primary mb-0">Предложенные записи</h4></div><span class="user_draft_count">1</span></div></a></div></div>',
    toast_info("Запись предложена"))
  }};
  link_.send(form_data);
});

on('#ajax', 'change', '#community_avatar_upload', function() {
  parent = this.parentElement;
  post_with_pk_and_reload(parent, "/gallery/community_progs/add_avatar/")
})

on('#ajax', 'click', '.c_post_fixed', function() {
  send_change(this, "/posts/community_progs/fixed/", "c_post_unfixed", "Открепить")
})
on('#ajax', 'click', '.c_post_unfixed', function() {
  send_change(this, "/posts/community_progs/unfixed/", "c_post_fixed", "Закрепить")
})

on('#ajax', 'click', '.c_post_off_comment', function() {
  send_change(this, "/posts/community_progs/off_comment/", "c_post_on_comment", "Вкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_post_comments") ? post.querySelector(".load_post_comments").style.display = "unset"
  : post.querySelector(".c_news_item_comments").style.display = "none"
});
on('#ajax', 'click', '.c_post_on_comment', function() {
  send_change(this, "/posts/community_progs/on_comment/", "c_post_off_comment", "Выкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_post_comments") ? post.querySelector(".load_post_comments").style.display = "unset"
  : post.querySelector(".c_news_item_comments").style.display = "unset"
})

on('#ajax', 'click', '.c_post_off_votes', function() {
  send_change(this, "/posts/community_progs/off_votes/", "c_post_on_votes", "Вкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "none";
  post.querySelector(".dislike").style.display = "none";
})
on('#ajax', 'click', '.c_post_on_votes', function() {
  send_change(this, "/posts/community_progs/on_votes/", "c_post_off_votes", "Выкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "unset";
  post.querySelector(".dislike").style.display = "unset";
})

on('#ajax', 'click', '.c_post_remove', function() {
  item = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  pk = item.getAttribute("data-pk");
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/posts/community_progs/delete/" + pk + "/", true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    p = document.createElement("div");
    p.classList.add("card", "mb-3");
    p.style.padding = "20px";
    p.innerHTML = "<span class='c_post_restore pointer' data-pk='" + pk + "'>Запись удалена. <span class='underline'>Восстановить</span></span>";
    !document.querySelector(".post_detail") ? (item.parentElement.insertBefore(p, item), item.style.display = "none")
    : (document.querySelector(".item_fullscreen").style.display = "none",
    block = document.body.querySelector(".post_stream"),
    item = block.querySelector( '[data-pk=' + '"' + pk + '"' + ']' ),
    item.parentElement.insertBefore(p, item),
    item.style.display = "none",
    p.style.display =  "block")
  }};

  link.send( );
});

on('#ajax', 'click', '.c_post_restore', function() {
  item = this.parentElement.nextElementSibling;
  pk = this.getAttribute("data-pk");
  block = this.parentElement;
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/posts/community_progs/restore/" + pk + "/", true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    block.remove();
    item.style.display = "block";
  }};

  link.send();
});

on('#ajax', 'change', '#c_photo_post_comment_attach', function() {
  pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");
  form_data = new FormData(document.body.querySelector("#add_photos"));
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/gallery/community_progs/add_comment_photo/" + pk + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    response = document.createElement("span");
    response.innerHTML = elem;
    photo_comment_attach(response.querySelectorAll(".c_photo_detail"), dropdown = document.body.querySelector(".current_file_dropdown").parentElement.parentElement, photo_list.length);
    }
    close_work_fullscreen();
  }
  link_.send(form_data);
});
