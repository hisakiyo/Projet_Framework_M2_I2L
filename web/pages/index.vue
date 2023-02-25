<template>
  <Login />
</template>

<script>
export default {
  name: 'IndexPage',
  middleware: 'need_auth',
}
</script>
