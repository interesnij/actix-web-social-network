on('#video_loader', 'click', '.c_video_off_comment', function() {
  send_photo_change(this, "/video/community_progs/off_comment/", "c_video_on_comment", "Вкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_video_comments").style.display = "none"
});
on('#video_loader', 'click', '.c_video_on_comment', function() {
  send_photo_change(this, "/video/community_progs/on_comment/", "c_video_off_comment", "Выкл. комментарии");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".load_video_comments").style.display = "unset"
});


on('#video_loader', 'click', '.c_video_off_votes', function() {
  send_photo_change(this, "/video/community_progs/off_votes/", "c_video_on_votes", "Вкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "none";
  post.querySelector(".dislike").style.display = "none";
});
on('#video_loader', 'click', '.c_video_on_votes', function() {
  send_photo_change(this, "/video/community_progs/on_votes/", "c_video_off_votes", "Выкл. реакции");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  post.querySelector(".like").style.display = "unset";
  post.querySelector(".dislike").style.display = "unset";
});

on('body', 'click', '.community_video_remove', function() {
  send_photo_change(this, "/video/community_progs/delete/", "community_video_restore", "Отмена");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  this.parentElement.parentElement.nextElementSibling.style.display = "none";
  post.querySelector(".order-2").style.display = "none";
  post.querySelector(".card").style.opacity = "0.5";
  this.style.color = "#FF0000";
});
on('body', 'click', '.community_video_restore', function() {
  send_photo_change(this, "/video/community_progs/restore/", "community_video_remove", "Удалить");
  post = this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement;
  this.parentElement.parentElement.nextElementSibling.style.display = "unset";
  post.querySelector(".order-2").style.display = "unset";
  post.querySelector(".card").style.opacity = "1";
});
