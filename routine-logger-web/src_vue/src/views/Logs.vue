<template>
  <v-container>
    <v-card>
      <v-card-title>
        Routine Logs
        <v-spacer></v-spacer>
      </v-card-title>
    </v-card>
  </v-container>
</template>

<script>
  import { mapGetters, mapState } from 'vuex'

  export default {
    data () {
      return {
      }
    },

    mounted() {
    },
    computed: {
        ...mapState({
        }),
    },
    methods: {
    }
  }
</script>
