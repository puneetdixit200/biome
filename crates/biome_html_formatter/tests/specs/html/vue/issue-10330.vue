<template>
<div>
<span>
<div>
<v-btn v-if="store.state.user" variant="text" to="/my-rooms">{{
	$t("nav.my-rooms")
}}</v-btn>
</div>
</span>
</div>
</template>

<template>
	<v-btn>
		{{
			store.state.room.enableVoteSkip
				? $t("room.next-video-vote")
				: $t("room.next-video")
		}}
	</v-btn>
</template>
