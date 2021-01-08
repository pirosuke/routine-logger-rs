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
        <template v-slot:item.actions="{ item }">
          <v-icon
            small
            class="mr-2"
            @click="showEditDialog(item)"
          >
            mdi-pencil
          </v-icon>
          <v-icon
            small
            @click="showDeleteDialog(item)"
          >
            mdi-delete
          </v-icon>
        </template>

        <template v-slot:top>
          <v-toolbar
            flat
          >
            <template>
              <v-spacer></v-spacer>
              <v-btn
                color="primary"
                dark
                class="mb-2"
                @click="showAddDialog"
              >
                新しいルーチンを追加
              </v-btn>
            </template>

            <v-dialog v-model="dialog" max-width="700px">
              <v-card>
                <v-card-title>
                  <span class="headline">ルーチンを追加</span>
                </v-card-title>

                <v-card-text>
                  <v-container>
                    <v-form
                      ref="form"
                      v-model="isEditedRoutineValid"
                      >
                      <v-row>
                        <v-col cols="12" sm="12" md="12">
                          <v-text-field 
                            :rules="[rules.required]"
                            v-model="editedItem.name" 
                            label="タイトル"
                            counter="255"
                          ></v-text-field>
                        </v-col>
                      </v-row>
                      <v-row>
                        <v-col cols="12" sm="3" md="3">
                          <v-text-field
                            :rules="[rules.required, rules.numOnly]"
                            v-model="editedItem.target_quantity"
                            label="目標回数"></v-text-field>
                        </v-col>
                        <v-col cols="12" sm="3" md="3">
                          <v-text-field v-model="editedItem.unit" label="単位"></v-text-field>
                        </v-col>
                        <v-col cols="12" sm="6" md="6">
                          <v-select
                            v-model="editedItem.unit_period"
                            label="単位期間"
                            item-text="name"
                            item-value="value"
                            :items="unit_periods"
                          ></v-select>
                        </v-col>
                      </v-row>
                    </v-form>
                  </v-container>
                </v-card-text>

                <v-card-actions>
                  <v-spacer></v-spacer>
                  <v-btn color="blue darken-1" @click="close">キャンセル</v-btn>
                  <v-btn color="blue darken-1" v-if="editedIndex !== -1" @click="updateItem">更新</v-btn>
                  <v-btn color="blue darken-1" v-if="editedIndex === -1" @click="addItem">登録</v-btn>
                </v-card-actions>
              </v-card>
            </v-dialog>

            <v-dialog v-model="dialogDeleteConfirm" max-width="500px">
              <v-card>
                <v-card-title>選択されたルーチン「{{editedItem.name}}」を削除します</v-card-title>
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
          { text: 'タイトル', value: 'name' },
          { text: '単位期間', value: 'unit_period' },
          { text: '目標回数', value: 'target_quantity' },
          { text: '単位', value: 'unit' },
          { text: '操作', value: 'actions', sortable: false},
        ],
        unit_periods: [
          {name: "日", value: "daily"},
          {name: "週", value: "weekly"},
          {name: "月", value: "monthly"},
        ],
        dialog: false,
        dialogDeleteConfirm: false,
        isEditedRoutineValid: true,
        editedIndex: -1,
        editedItem: {
        },
        defaultItem: {
          routine_id: 0,
          name: '',
          unit_period: 'daily',
          target_quantity: '1',
          unit: '回',
        },
        rules: {
          required: value => !!value || '必須項目です',
          numOnly: value => {
            const pattern = /^[0-9]*$/
            return pattern.test(value) || '数値以外の文字が含まれています'
          },
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
        this.$refs.form.resetValidation()
        this.dialog = false;
      },

      showAddDialog () {
        this.editedIndex = -1
        this.editedItem = Object.assign({}, this.defaultItem)
        this.dialog = true
      },

      addItem () {
        this.$refs.form.validate()
        if (this.isEditedRoutineValid) {
          this.$store.dispatch('addRoutine', {
            editedItem: this.editedItem,
          }).then(() => {
            this.$store.dispatch('fetchRoutines')
          })
          this.close();
        }
      },

      showEditDialog (item) {
        this.editedIndex = this.routines.indexOf(item)
        this.editedItem = Object.assign({}, item)
        this.dialog = true
      },

      updateItem () {
        this.$refs.form.validate()
        if (this.isEditedRoutineValid) {
          this.$store.dispatch('updateRoutine', {
            editedItem: this.editedItem,
          }).then(() => {
            this.$store.dispatch('fetchRoutines')
          })
          this.close();
        }
      },

      showDeleteDialog (item) {
        this.editedIndex = this.routines.indexOf(item)
        this.editedItem = Object.assign({}, item)
        this.dialogDeleteConfirm = true
      },

      closeDeleteConfirm () {
        this.dialogDeleteConfirm = false
      },

      deleteItem () {
        this.$store.dispatch('deleteRoutine', {
          editedItem: this.editedItem,
        }).then(() => {
          this.$store.dispatch('fetchRoutines')
        })
        this.closeDeleteConfirm()
      },
    }
  }
</script>
