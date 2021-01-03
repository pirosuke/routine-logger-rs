<template>
  <v-container>
    <v-card>
      <v-card-title>
        ルーチン管理
        <v-spacer></v-spacer>
      </v-card-title>


      <v-data-table
        :headers="headers"
        :items="routines"
      >
        <template v-slot:top>
          <v-toolbar
            flat
          >
            <v-dialog v-model="dialog" max-width="700px">
              <template v-slot:activator="{ on, attrs }">
                <v-btn
                  color="primary"
                  dark
                  class="mb-2"
                  v-bind="attrs"
                  v-on="on"
                >
                  新しいルーチンを追加
                </v-btn>
              </template>
              <v-card>
                <v-card-title>
                  <span class="headline">ルーチンを追加</span>
                </v-card-title>

                <v-card-text>
                  <v-container>
                    <v-row>
                      <v-col cols="12" sm="12" md="12">
                        <v-text-field 
                          v-model="editedItem.name" 
                          label="タイトル"
                          counter="255"
                        ></v-text-field>
                      </v-col>
                    </v-row>
                    <v-row>
                      <v-col cols="12" sm="6" md="6">
                        <v-select
                          v-model="editedItem.unit_period"
                          label="単位期間"
                          item-text="name"
                          item-value="value"
                          :items="unit_periods"
                        ></v-select>
                      </v-col>
                      <v-col cols="12" sm="6" md="6">
                        <v-text-field v-model="editedItem.target_quantity" label="目標回数"></v-text-field>
                      </v-col>
                    </v-row>

                  </v-container>
                </v-card-text>

                <v-card-actions>
                  <v-spacer></v-spacer>
                  <v-btn color="blue darken-1" @click="close">キャンセル</v-btn>
                  <v-btn color="blue darken-1" @click="save">登録</v-btn>
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
          { text: 'タイトル', value: 'name' },
          { text: '単位期間', value: 'unit_period' },
          { text: '目標回数', value: 'target_quantity' },
        ],
        unit_periods: [
          {name: "日", value: "daily"},
          {name: "週", value: "weekly"},
          {name: "月", value: "monthly"},
        ],
        dialog: false,
        editedItem: {
        },
      }
    },

    watch: {
      dialog (val) {
        val || this.close();
      },
    },

    mounted() {
      this.$store.dispatch('fetchRoutines')
    },
    computed: {
        ...mapState({
          routines: state => state.routines,
        })
    },
    methods: {
      close () {
        this.dialog = false;
      },

      save () {
        //console.log(this.editedItem);
        this.close();
        this.$store.dispatch('addRoutine', {
          editedItem: this.editedItem,
        })
      },
    }
  }
</script>
