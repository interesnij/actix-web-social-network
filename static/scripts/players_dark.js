
		music_player = 	new FWDMSP({
				//main settings
				instanceName:"player1",
				playlistsId:"audio_playlists",
				mainFolderPath:"/static/images/",
				skinPath:"audio_dark",
				showSoundCloudUserNameInTitle:"no",
				showMainBackground:"yes",
				verticalPosition:"bottom",
				useDeepLinking:"no",
				rightClickContextMenu:"developer",
				addKeyboardSupport:"yes",
				animate:"yes",
				autoPlay:"no",
				loop:"no",
				shuffle:"no",
				maxWidth:850,
				volume:.8,
				//controller settings
				showControllerByDefault:"yes",
				showThumbnail:"yes",
				showNextAndPrevButtons:"yes",
				showSoundAnimation:"yes",
				showLoopButton:"yes",
				showShuffleButton:"yes",
				showBuyButton:"yes",
				expandBackground:"no",
				titleColor:"#FFFFFF",
				timeColor:"#888888",

				//controller align and size settings (described in detail in the documentation!)
				controllerHeight:76,
				startSpaceBetweenButtons:9,
				spaceBetweenButtons:8,
				separatorOffsetOutSpace:5,
				separatorOffsetInSpace:9,
				lastButtonsOffsetTop:14,
				allButtonsOffsetTopAndBottom:14,
				titleBarOffsetTop:13,
				mainScrubberOffsetTop:47,
				spaceBetweenMainScrubberAndTime:10,
				startTimeSpace:10,
				scrubbersOffsetWidth:2,
				scrubbersOffestTotalWidth:0,
				volumeButtonAndScrubberOffsetTop:47,
				spaceBetweenVolumeButtonAndScrubber:6,
				volumeScrubberOffestWidth:4,
				scrubberOffsetBottom:10,
				equlizerOffsetLeft:1,
				//playlists window settings
				showPlaylistsSearchInput:"no",
				usePlaylistsSelectBox:"no",
				showPlaylistsSelectBoxNumbers:"no",
				showPlaylistsButtonAndPlaylists:"yes",
				showPlaylistsByDefault:"no",
				thumbnailSelectedType:"opacity",
				startAtPlaylist:0,
				startAtTrack:0,
				startAtRandomTrack:"no",
				buttonsMargins:0,
				thumbnailMaxWidth:330,
				thumbnailMaxHeight:330,
				horizontalSpaceBetweenThumbnails:40,
				verticalSpaceBetweenThumbnails:40,
				mainSelectorBackgroundSelectedColor:"#FFFFFF",
				mainSelectorTextNormalColor:"#FFFFFF",
				mainSelectorTextSelectedColor:"#000000",
				mainButtonTextNormalColor:"#888888",
				mainButtonTextSelectedColor:"#FFFFFF",
				//playlist settings
				playTrackAfterPlaylistLoad:"no",
				showPlayListButtonAndPlaylist:"yes",
				showPlayListOnAndroid:"yes",
				showPlayListByDefault:"no",
				showPlaylistItemPlayButton:"yes",
				showPlaylistItemBuyButton:"no",
				addScrollBarMouseWheelSupport:"yes",
				showTracksNumbers:"yes",
				playlistBackgroundColor:"#000000",
				trackTitleNormalColor:"#888888",
				trackTitleSelectedColor:"#FFFFFF",
				trackDurationColor:"#888888",
				maxPlaylistItems:30,
				nrOfVisiblePlaylistItems:12,
				trackTitleOffsetLeft:0,
				playPauseButtonOffsetLeftAndRight:11,
				durationOffsetRight:9,
				scrollbarOffestWidth:7,
				//playback rate / speed
				showPlaybackRateButton:"yes",
				defaultPlaybackRate:1, //min - 0.5 / max - 3
				playbackRateWindowTextColor:"#888888",
				//search bar settings
				showSearchBar:"no",
				showSortButtons:"yes",
				searchInputColor:"#999999",
				searchBarHeight:38,
				inputSearchTextOffsetTop:1,
				inputSearchOffsetLeft:0,

				//opener settings
				openerAlignment:"right",
				showOpener:"yes",
				showOpenerPlayPauseButton:"yes",
				openerEqulizerOffsetLeft:3,
				openerEqulizerOffsetTop:-1,
				//popup settings
				showPopupButton:"no",
				popupWindowBackgroundColor:"#878787",
				popupWindowWidth:850,
				popupWindowHeight:423,
				//a to b loop
				atbTimeBackgroundColor:"transparent",
				atbTimeTextColorNormal:"#888888",
				atbTimeTextColorSelected:"#FFFFFF",
				atbButtonTextNormalColor:"#888888",
				atbButtonTextSelectedColor:"#FFFFFF",
				atbButtonBackgroundNormalColor:"#FFFFFF",
				atbButtonBackgroundSelectedColor:"#000000",
			});



if (document.querySelector("#video_player")) {
video_player = new FWDUVPlayer({
//main settings
instanceName:"player_dark",
parentId:"video_player",
playlistsId:"video_playlists",
mainFolderPath:"/static",
skinPath:"images/video_dark/",
displayType:"responsive",
					initializeOnlyWhenVisible:"no",
					useVectorIcons:"no",
					fillEntireVideoScreen:"no",
					fillEntireposterScreen:"yes",
					goFullScreenOnButtonPlay:"no",
					playsinline:"yes",
					privateVideoPassword:"428c841430ea18a70f7b06525d4b748a",
					youtubeAPIKey:'AIzaSyCgbixU3aIWCkiZ76h_E-XpEGig5mFhnVY',
					useHEXColorsForSkin:"no",
					normalHEXButtonsColor:"#666666",
					useDeepLinking:"yes",
					googleAnalyticsTrackingCode:"",
					useResumeOnPlay:"no",
					showPreloader:"yes",
					preloaderBackgroundColor:"#000000",
					preloaderFillColor:"#FFFFFF",
					addKeyboardSupport:"yes",
					autoScale:"yes",
					stopVideoWhenPlayComplete:"no",
					playAfterVideoStop:"no",
					autoPlay:"no",
					loop:"no",
					shuffle:"no",
					showErrorInfo:"yes",
					maxWidth:1170,
					maxHeight:659,
					volume:.8,
					backgroundColor:"#000000",
					videoBackgroundColor:"#000000",
					posterBackgroundColor:"#000000",
					//logo settings
					showLogo:"yes",
					hideLogoWithController:"yes",
					logoPosition:"topRight",
					logoLink:"",
					logoMargins:10,
					//playlists/categories settings
					usePlaylistsSelectBox:"yes",
					showPlaylistsButtonAndPlaylists:"yes",
					showPlaylistsByDefault:"no",
					thumbnailSelectedType:"opacity",
					startAtPlaylist:0,
					buttonsMargins:15,
					thumbnailMaxWidth:350,
					thumbnailMaxHeight:350,
					horizontalSpaceBetweenThumbnails:40,
					verticalSpaceBetweenThumbnails:40,
					inputBackgroundColor:"#333333",
					inputColor:"#999999",
					//playlist settings
					showPlaylistButtonAndPlaylist:"yes",
					playlistPosition:"right",
					showPlaylistByDefault:"yes",
					showPlaylistName:"yes",
					showLoopButton:"yes",
					showShuffleButton:"yes",
					showPlaylistOnFullScreen:"no",
					showNextAndPrevButtons:"yes",
					showThumbnail:"yes",
					addMouseWheelSupport:"yes",
					startAtRandomVideo:"no",
					stopAfterLastVideoHasPlayed:"no",
					addScrollOnMouseMove:"no",
					randomizePlaylist:'no',
					folderVideoLabel:"VIDEO ",
					playlistRightWidth:320,
					playlistBottomHeight:380,
					startAtVideo:0,
					maxPlaylistItems:50,
					thumbnailWidth:71,
					thumbnailHeight:71,
					spaceBetweenControllerAndPlaylist:1,
					spaceBetweenThumbnails:1,
					scrollbarOffestWidth:8,
					scollbarSpeedSensitivity:.5,
					playlistBackgroundColor:"#000000",
					playlistNameColor:"#FFFFFF",
					thumbnailNormalBackgroundColor:"#1b1b1b",
					thumbnailHoverBackgroundColor:"#313131",
					thumbnailDisabledBackgroundColor:"#272727",
					youtubeAndFolderVideoTitleColor:"#FFFFFF",
					folderAudioSecondTitleColor:"#999999",
					youtubeOwnerColor:"#888888",
					youtubeDescriptionColor:"#888888",
					mainSelectorBackgroundSelectedColor:"#FFFFFF",
					mainSelectorTextNormalColor:"#FFFFFF",
					mainSelectorTextSelectedColor:"#000000",
					mainButtonBackgroundNormalColor:"#212021",
					mainButtonBackgroundSelectedColor:"#FFFFFF",
					mainButtonTextNormalColor:"#FFFFFF",
					mainButtonTextSelectedColor:"#000000",
					//controller settings
					showController:"yes",
					showControllerWhenVideoIsStopped:"yes",
					showNextAndPrevButtonsInController:"no",
					showRewindButton:"yes",
					showPlaybackRateButton:"yes",
					showVolumeButton:"yes",
					showTime:"yes",
					showQualityButton:"yes",
					showInfoButton:"yes",
					showEmbedButton:"yes",
					showChromecastButton:"no",
					showFullScreenButton:"yes",
					disableVideoScrubber:"no",
					showScrubberWhenControllerIsHidden:"yes",
					showDefaultControllerForVimeo:"no",
					repeatBackground:"yes",
					controllerHeight:42,
					controllerHideDelay:3,
					startSpaceBetweenButtons:7,
					spaceBetweenButtons:8,
					scrubbersOffsetWidth:2,
					mainScrubberOffestTop:14,
					timeOffsetLeftWidth:5,
					timeOffsetRightWidth:3,
					timeOffsetTop:0,
					volumeScrubberHeight:80,
					volumeScrubberOfsetHeight:12,
					timeColor:"#888888",
					youtubeQualityButtonNormalColor:"#888888",
					youtubeQualityButtonSelectedColor:"#FFFFFF",
					//advertisement on pause window
					aopwTitle:"Advertisement",
					aopwWidth:400,
					aopwHeight:240,
					aopwBorderSize:6,
					aopwTitleColor:"#FFFFFF",
					//subtitle
					subtitlesOffLabel:"Subtitle off",
					//popup add windows
					showPopupAdsCloseButton:"yes",
					//embed window and info window
					embedAndInfoWindowCloseButtonMargins:15,
					borderColor:"#333333",
					mainLabelsColor:"#FFFFFF",
					secondaryLabelsColor:"#a1a1a1",
					inputBackgroundColor:"#000000",
					inputColor:"#FFFFFF",
					//loggin
					isLoggedIn:"yes",
					playVideoOnlyWhenLoggedIn:"yes",
					loggedInMessage:"Please login to view this video.",
					//audio visualizer
					audioVisualizerLinesColor:"#ff9f00",
					audioVisualizerCircleColor:"#FFFFFF",
					//lightbox settings
					lightBoxBackgroundOpacity:.6,
					lightBoxBackgroundColor:"#000000",
					//sticky on scroll
					stickyOnScroll:"no",
					stickyOnScrollShowOpener:"yes",
					stickyOnScrollWidth:"700",
					stickyOnScrollHeight:"394",
					//sticky display settings
					showOpener:"yes",
					showOpenerPlayPauseButton:"yes",
					verticalPosition:"bottom",
					horizontalPosition:"center",
					showPlayerByDefault:"yes",
					animatePlayer:"yes",
					openerAlignment:"right",
					mainBackgroundImagePath:"content/minimal_skin_dark/main-background.png",
					openerEqulizerOffsetTop:-1,
					openerEqulizerOffsetLeft:3,
					offsetX:0,
					offsetY:0,
					//playback rate / speed
					defaultPlaybackRate:1, //0.25, 0.5, 1, 1.25, 1.2, 2
					//cuepoints
					executeCuepointsOnlyOnce:"no",
					//annotations
					showAnnotationsPositionTool:"no",
					//ads
					openNewPageAtTheEndOfTheAds:"no",
					playAdsOnlyOnce:"no",
					adsButtonsPosition:"left",
					skipToVideoText:"You can skip to video in: ",
					skipToVideoButtonText:"Skip Ad",
					adsTextNormalColor:"#888888",
					adsTextSelectedColor:"#FFFFFF",
					adsBorderNormalColor:"#666666",
					adsBorderSelectedColor:"#FFFFFF",
					//a to b loop
					useAToB:"yes",
					atbTimeBackgroundColor:"transparent",
					atbTimeTextColorNormal:"#888888",
					atbTimeTextColorSelected:"#FFFFFF",
					atbButtonTextNormalColor:"#888888",
					atbButtonTextSelectedColor:"#FFFFFF",
					atbButtonBackgroundNormalColor:"#FFFFFF",
					atbButtonBackgroundSelectedColor:"#000000",
					//thumbnails preview
					thumbnailsPreviewWidth:196,
					thumbnailsPreviewHeight:110,
					thumbnailsPreviewBackgroundColor:"#000000",
					thumbnailsPreviewBorderColor:"#666",
					thumbnailsPreviewLabelBackgroundColor:"#666",
					thumbnailsPreviewLabelFontColor:"#FFF",
					// context menu
					showContextmenu:'yes',
					showScriptDeveloper:"no",
					contextMenuBackgroundColor:"#1f1f1f",
					contextMenuBorderColor:"#1f1f1f",
					contextMenuSpacerColor:"#333",
					contextMenuItemNormalColor:"#888888",
					contextMenuItemSelectedColor:"#FFFFFF",
					contextMenuItemDisabledColor:"#444"
})

FWDUVPUtils.onReady(function(){
    video_player.addListener(FWDUVPlayer.READY, video_onReady);
    video_player.addListener(FWDUVPlayer.PLAY, video_onPlay);
});
}
FWDMSPUtils.onReady(function(){
        music_player.addListener(FWDMSP.READY, music_onReady);
        music_player.addListener(FWDMSP.PLAY, music_onPlay);
    });

function music_onReady(){console.log("Аудио плеер готов");}
function video_onReady(){console.log("Видео плеер готов");}

function video_onPlay(){
    console.log("Воспроизводится видео №: " + video_player.getVideoId());
    music_player.pause();
}
function music_onPlay(){
    console.log("Воспроизводится трек № : " + music_player.getTrackId());
    title = music_player.getTrackTitle();
    document.title = title;
    try{video_player.pause();}catch{var a=0}
};

function save_playlist(suffix, post_link, get_link, track_id){
    var playlist_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    playlist_link.open( 'GET', post_link, true );
    playlist_link.onreadystatechange = function () {
    if ( playlist_link.readyState == 4 && playlist_link.status == 200 ) {
      document.querySelector("body").className = "";
      document.querySelector("body").classList.add(suffix);

      var _link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      _link.open( 'GET', get_link, true );
      _link.onreadystatechange = function () {
        if ( _link.readyState == 4 && _link.status == 200 ) {
          var response = document.createElement('span');
          response.innerHTML = _link.responseText;
          var list = response.querySelectorAll("li");
          var count = list.length;
          for(i=0; i<count; i++) {
            _source=list[i].getAttribute("data-path") + '/stream?client_id=' + 'dce5652caa1b66331903493735ddd64d';
            _title=list[i].getAttribute("data-title");
            _thumbPath=list[i].getAttribute("data-thumbpath");
            _duration=list[i].getAttribute("data-duration");
            time = msToTime(_duration);
            music_player.addTrack(_source, _title, _thumbPath, time, true, false, null);
          }
          music_player.loadPlaylist(0);
          if (FWDMSP.LOAD_PLAYLIST_COMPLETE){
            console.log("Плейдист загружен!");
          setTimeout(function() {music_player.playSpecificTrack(suffix, track_id)}, 50);
        }
      }};
      _link.send( null );
    }};
    playlist_link.send( null );
    };


on('#ajax', 'click', '.tag_item', function() {
  var track_id = this.parentElement.parentElement.getAttribute('music-counter');
  var tag_pk = document.querySelector(".tag_playlist").getAttribute('data-pk');
  if (!document.body.classList.contains("tag_" + tag_pk)){
    save_playlist("tag_" + tag_pk, '/music/manage/temp_tag/' + tag_pk, '/music/get/tag/' + tag_pk, track_id)
  }else{
    music_player.loadPlaylist(0);
    if (FWDMSP.LOAD_PLAYLIST_COMPLETE){
      console.log("Плейдист загружен!");
    setTimeout(function() {music_player.playSpecificTrack("tag_" + tag_pk, track_id)}, 50);
  }
  }
  });

on('#ajax', 'click', '.genre_item', function() {
  var track_id = this.parentElement.parentElement.getAttribute('music-counter');
  var genre_pk = document.querySelector(".genre_playlist").getAttribute('data-pk');
  if (!document.body.classList.contains("genre_" + genre_pk)){
    save_playlist("genre_" + genre_pk, '/music/manage/temp_genre/' + genre_pk, '/music/get/genre/' + genre_pk, track_id)
  }else{
    music_player.loadPlaylist(0);
    if (FWDMSP.LOAD_PLAYLIST_COMPLETE){
      console.log("Плейлист загружен!");
    setTimeout(function() {music_player.playSpecificTrack("genre_" + list_pk, track_id)}, 50);
  }
  }
})


on('#ajax', 'click', '#load_1', function(e) {
  music_player.loadPlaylist(0);
})
on('#ajax', 'click', '#load_2', function(e) {
  music_player.loadPlaylist(1);
})
on('#ajax', 'click', '#load_3', function(e) {
  music_player.loadPlaylist(2);
})

on('#ajax', 'click', '.track_add', function(e) {
  block = this.parentElement;
  pk = block.parentElement.getAttribute("data-pk");
  var _link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  _link.open( 'GET', "/music/manage/add_track/" + pk, true );
  _link.onreadystatechange = function () {
    if ( _link.readyState == 4 && _link.status == 200 ) {
      block.innerHTML = "";
      block.innerHTML = "<span class='track_remove' title='Удалить'><svg xmlns='http://www.w3.org/2000/svg' fill='currentColor' style='width:22px;height:22px;' class='svg_default' viewBox='0 0 2424'><path fill='none' d='M0 0h24v24H0z'/><path d='M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z'/></svg></span>"
  }};
  _link.send( null );
});

on('#ajax', 'click', '.track_remove', function(e) {
  block = this.parentElement;
  pk = block.parentElement.getAttribute("data-pk");
  var _link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  _link.open( 'GET', "/music/manage/remove_track/" + pk, true );
  _link.onreadystatechange = function () {
    if ( _link.readyState == 4 && _link.status == 200 ) {
      block.innerHTML = "";
      block.innerHTML = "<span class='track_add' title='Добавить'><svg fill='currentColor' style='width:22px;height:22px;' class='svg_default' xmlns='http://www.w3.org/2000/svg' viewBox='0 024 24'><path d='M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z'/><path d='M0 0h24v24H0z' fill='none'/></svg></span>"
  }};
  _link.send( null );
});
