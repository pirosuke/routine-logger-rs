<template>
  <v-container>
    <v-card>
      <v-card-title>
        ルーチン実施履歴
        <v-spacer></v-spacer>
      </v-card-title>

      <v-data-table
        :headers="headers"
        :items="routineLogs"
        :disable-sort=true
        :options.sync="fetchOptions"
        :server-items-length="logCount"
        :loading="loading"
        :footer-props="{
          'items-per-page-options': [10,20,30,50]
        }"
      >
        <template v-slot:item.actions="{ item }">
          <v-menu
            bottom
            offset-y
          >
            <template v-slot:activator="{ on, attrs }">
              <v-btn
                icon
                class="ma-2"
                v-bind="attrs"
                v-on="on"
              >
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>
            <v-list>
              <v-list-item
                @click="showDeleteDialog(item)"
              >
                <v-list-item-title>ログを削除する</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </template>

        <template v-slot:top>
          <v-toolbar
            flat
          >
            <v-dialog v-model="dialogDeleteConfirm" max-width="500px">
              <v-card>
                <v-card-title>選択されたログ「{{editedItem.date_of_activity}}」を削除します</v-card-title>
                <v-card-actions>
                  <v-spacer></v-spacer>
                  <v-btn color="blue darken-1" text @click="closeDeleteConfirm">キャンセル</v-btn>
                  <v-btn color="red darken-1" text @click="deleteItem">削除する</v-btn>
                </v-card-actions>
              </v-card>
            </v-dialog>

          </v-toolbar>
        </template>

      </v-data-table>
    </v-card>
  </v-container>
</template>

<script>
  import { mapGetters, mapState } from 'vuex'

  export default {
    data () {
      return {
        headers: [
          { text: '実施日', value: 'date_of_activity' },
          { text: 'ルーチン', value: 'routine_name' },
          { text: '実施回数', value: 'quantity' },
          { text: '単位', value: 'unit' },
          { text: '操作', value: 'actions', sortable: false},
        ],
        dialogDeleteConfirm: false,
        editedIndex: -1,
        editedItem: {
        },
        loading: false,
        logCount: 0,
        fetchOptions: {},
      }
    },

    mounted() {
    },
    computed: {
        ...mapState({
        }),
    },
    watch: {
      fetchOptions: {
        handler () {
          this.getLogData()
        },
        deep: true
      }
    },
    methods: {
      getLogData () {
        this.loading = true;

        const { sortBy, sortDesc, page, itemsPerPage } = this.fetchOptions;

        this.$store.dispatch('routineLogs/fetchLogs', {
          page,
          itemsPerPage,
        }).then(() => {
          this.routineLogs = this.$store.state.routineLogs.logs;
          this.logCount = this.$store.state.routineLogs.logCount;
          this.loading = false;
        });
      },

      showDeleteDialog (item) {
        this.editedIndex = this.routineLogs.indexOf(item)
        this.editedItem = Object.assign({}, item)
        this.dialogDeleteConfirm = true
      },

      closeDeleteConfirm () {
        this.dialogDeleteConfirm = false
      },

      deleteItem () {
        this.$store.dispatch('routineLogs/deleteLog', {
          editedItem: this.editedItem,
        }).then(() => {
          this.getLogData()
        })
        this.closeDeleteConfirm()
      },
    }
  }
</script>
