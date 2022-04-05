on('#ajax', 'click', '.c_good_add_attach', function() {
  pk = document.body.querySelector(".pk_saver").getAttribute('data-pk');
  create_fullscreen('/goods/community_progs/add_attach/' + pk + '/', "item_fullscreen");
});


on('#ajax', 'click', '.c_good_off_comment', function() {
  send_good_change(this, "/goods/community_progs/off_comment/", "c_good_on_comment", "Вкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_good_comments").style.display = "none"
});
on('#ajax', 'click', '.c_good_on_comment', function() {
  send_good_change(this, "/goods/community_progs/on_comment/", "c_good_off_comment", "Выкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_good_comments").style.display = "unset"
});


on('#ajax', 'click', '.c_good_add', function() {
  pk = document.body.querySelector(".pk_saver").getAttribute('data-pk');
  create_fullscreen('/goods/community_progs/add/' + pk + '/', "item_fullscreen");
});

on('#ajax', 'click', '.c_good_off_votes', function() {
  send_good_change(this, "/goods/community_progs/off_votes/", "c_good_on_votes", "Вкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "none";
  post.querySelector(".dislike").style.display = "none";
});
on('#ajax', 'click', '.c_good_on_votes', function() {
  send_good_change(this, "/goods/community_progs/on_votes/", "c_good_off_votes", "Выкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "unset";
  post.querySelector(".dislike").style.display = "unset";
});
on('#ajax', 'click', '.c_good_hide', function() {
  send_good_change(this, "/goods/community_progs/hide/", "u_good_unhide", "Товар не виден");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
});
on('#ajax', 'click', '.c_good_unhide', function() {
  send_good_change(this, "/goods/community_progs/unhide/", "u_good_hide", "Товар виден");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
});

on('#ajax', 'click', '.community_good_remove', function() {
  send_good_change(this, "/goods/community_progs/delete/", "community_good_restore", "Отмена");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  this.parentElement.parentElement.nextElementSibling.style.display = "none";
  post.querySelector(".order-2").style.display = "none";
  post.querySelector(".card").style.opacity = "0.5";
  this.style.color = "#FF0000";
});
on('#ajax', 'click', '.community_good_restore', function() {
  send_good_change(this, "/goods/community_progs/restore/", "community_good_remove", "Удалить");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  this.parentElement.parentElement.nextElementSibling.style.display = "unset";
  post.querySelector(".order-2").style.display = "unset";
  post.querySelector(".card").style.opacity = "1";
});

on('#ajax', 'click', '#c_add_good_btn', function() {
  form_post = document.body.querySelector("#c_add_good_form");
  form_data = new FormData(form_post);

  lists = form_post.querySelector("#id_list");
  selectedOptions = lists.selectedOptions;
  val = false;
  for (var i = 0; i < selectedOptions.length; i++) {
    if(selectedOptions[i].value) {val = true}
  }
  if (!document.body.querySelector("#id_title").value){
    document.body.querySelector("#id_title").style.border = "1px #FF0000 solid";
    toast_error("Название - обязательное поле!");
  } else if (!document.body.querySelector("#category").value){
    document.body.querySelector("#category").style.border = "1px #FF0000 solid";
    toast_error("Категория - обязательное поле!")
  } else if (!document.body.querySelector("#id_description").value){
    document.body.querySelector("#id_description").style.border = "1px #FF0000 solid";
    toast_error("Описание товара - обязательное поле!");
  } else if (!document.body.querySelector("#id_image").value){
    document.body.querySelector("#good_image").style.border = "1px #FF0000 solid !important";
    toast_error("Фотография на обложку обязательна!")
  } else if (!val){
    form_post.querySelector("#id_list").style.border = "1px #FF0000 solid";
    toast_error("Выберите альбом!");
    return
  } else {this.disabled = true}

  pk = document.body.querySelector(".pk_saver").getAttribute("data-pk");
  link_ = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link_.open( 'POST', "/goods/community_progs/add/" + pk + "/", true );
  link_.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link_.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
    elem = link_.responseText;
    new_good = document.createElement("div");
    new_good.innerHTML = elem;
    if (document.querySelector(".current_file_dropdown")){
      dropdown = document.body.querySelector(".current_file_dropdown").parentElement.parentElement;
      is_full_dropdown();
      img_block = dropdown.parentElement.previousElementSibling;
      data_pk = new_good.querySelector(".new_image").getAttribute('good-pk');
      new_good.classList.add("attach_toggle");
      title = new_good.querySelector(".good_title").innerHTML;
      div = create_preview_good(new_good.querySelector(".img").getAttribute('data-src'), data_pk, title);
      img_block.append(div);
      add_file_dropdown()
      is_full_dropdown();

    } else if (document.querySelector(".attach_block")){
      block = document.body.querySelector(".attach_block");
      is_full_attach();
      data_pk = new_good.querySelector(".new_image").getAttribute('good-pk');
      title = new_good.querySelector(".good_title").innerHTML;

      new_good.classList.add("attach_toggle");
      div = create_preview_good(new_good.querySelector(".img").getAttribute('data-src'), data_pk, title);
    block.append(div);
    add_file_attach()
    is_full_attach();
    }
    else {
      uuid = document.body.querySelector(".pk_saver").getAttribute("data-uuid");
      span1 = new_good.querySelector('.span1')
      if (span1.classList.contains(uuid)){
        container = document.body.querySelector(".is_paginate");
        container.insertAdjacentHTML('afterBegin', new_good.innerHTML);
        container.querySelector(".items_empty") ? container.querySelector(".items_empty").style.display = "none" : null;
      }
  };
  close_work_fullscreen();
  toast_info("Товар создан!")
  }};
  link_.send(form_data);
});

on('#ajax', 'click', '.mob_c_good_off_comment', function() {
  send_mob_good_change(this, "/goods/community_progs/off_comment/", "mob_c_good_on_comment", "Вкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_good_comments").style.display = "none"
});
on('#ajax', 'click', '.mob_c_good_on_comment', function() {
  send_mob_good_change(this, "/goods/community_progs/on_comment/", "mob_c_good_off_comment", "Выкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_good_comments").style.display = "unset"
});
on('#ajax', 'click', '.mob_c_good_off_votes', function() {
  send_mob_good_change(this, "/goods/community_progs/off_votes/", "mob_c_good_on_votes", "Вкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "none";
  post.querySelector(".dislike").style.display = "none";
});
on('#ajax', 'click', '.mob_c_good_on_votes', function() {
  send_mob_good_change(this, "/goods/community_progs/on_votes/", "mob_c_good_off_votes", "Выкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "unset";
  post.querySelector(".dislike").style.display = "unset";
});
on('#ajax', 'click', '.mob_c_good_hide', function() {
  send_mob_good_change(this, "/goods/community_progs/hide/", "mob_c_good_unhide", "Товар не виден");
});
on('#ajax', 'click', '.mob_c_good_unhide', function() {
  send_mob_good_change(this, "/goods/community_progs/unhide/", "mob_c_good_hide", "Товар виден");
});
on('#ajax', 'click', '.mob_user_good_remove', function() {
  send_mob_good_change(this, "/goods/user_progs/delete/", "mob_community_progs_good_restore", "Отмена");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".good_card").style.display = "none";
  post.querySelector(".card").style.opacity = "0.5";
  this.style.color = "#FF0000";
});
on('#ajax', 'click', '.mob_community_good_restore', function() {
  send_mob_good_change(this, "/goods/community_progs/restore/", "mob_community_good_remove", "Удалить");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".good_card").style.display = "unset";
  post.querySelector(".card").style.opacity = "1";
});
