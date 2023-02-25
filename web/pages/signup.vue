<template>
  <Signup />
</template>

<script>
export default {
  name: 'SignupPage',
  middleware: 'need_auth',
}
</script>
