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
          <v-btn
            color="primary"
            class="ma-2"
            outlined
            @click="showLogAddDialog(item)"
          >
            <v-icon
              left
            >mdi-plus-box</v-icon>
            ログを追加
          </v-btn>
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
                @click="showEditDialog(item)"
              >
                <v-list-item-title>ルーチンを編集する</v-list-item-title>
              </v-list-item>
              <v-list-item
                @click="showDeleteDialog(item)"
              >
                <v-list-item-title>ルーチンを削除する</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
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

            <v-dialog v-model="dialogLogAdd" max-width="700px">
              <v-card>
                <v-card-title>
                  <span class="headline">{{editedItem.name}}ログを追加</span>
                </v-card-title>

                <v-card-text>
                  <v-container>
                    <v-form
                      ref="logAddForm"
                      v-model="isEditedLogValid"
                      >
                      <v-row>
                        <v-col cols="6" sm="6" md="6">
                          <v-menu
                            ref="dateDialog"
                            v-model="dateDialog"
                            :close-on-content-click="false"
                            transition="scale-transition"
                            offset-y
                            min-width="290px"
                          >
                            <template v-slot:activator="{on, attrs}">
                              <v-text-field
                                label="実施日"
                                :rules="[rules.required]"
                                prepend-icon="mdi-calendar"
                                readonly
                                v-bind="attrs"
                                v-on="on"
                                v-model="editedLog.date_of_activity"
                              ></v-text-field>
                            </template>
                            <v-date-picker
                              v-model="editedLog.date_of_activity"
                              @input="dateDialog = false"
                              no-title
                              scrollable
                            >
                            </v-date-picker>
                          </v-menu>
                        </v-col>
                        <v-col cols="6" sm="6" md="6">
                          <v-text-field
                            :rules="[rules.required, rules.numOnly]"
                            v-model="editedLog.quantity"
                            :suffix="editedItem.unit"
                            label="回数"></v-text-field>
                        </v-col>
                      </v-row>
                    </v-form>
                  </v-container>
                </v-card-text>

                <v-card-actions>
                  <v-spacer></v-spacer>
                  <v-btn color="blue darken-1" @click="closeLogAddDialog">キャンセル</v-btn>
                  <v-btn color="blue darken-1" @click="addLog">追加する</v-btn>
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
        dialogLogAdd: false,
        isEditedRoutineValid: true,
        isEditedLogValid: true,
        editedIndex: -1,
        editedItem: {
        },
        editedLog: {
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
            const pattern = /^[0-9\.]*$/
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
      this.$store.dispatch('routines/fetchRoutines')
    },
    computed: {
        ...mapState({
          routines: state => state.routines.routines,
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
          this.$store.dispatch('routines/addRoutine', {
            editedItem: this.editedItem,
          }).then(() => {
            this.$store.dispatch('routines/fetchRoutines')
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
          this.$store.dispatch('routines/updateRoutine', {
            editedItem: this.editedItem,
          }).then(() => {
            this.$store.dispatch('routines/fetchRoutines')
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
        this.$store.dispatch('routines/deleteRoutine', {
          editedItem: this.editedItem,
        }).then(() => {
          this.$store.dispatch('routines/fetchRoutines')
        })
        this.closeDeleteConfirm()
      },

      showLogAddDialog (item) {
        this.editedIndex = this.routines.indexOf(item)
        this.editedItem = Object.assign({}, item)
        this.dialogLogAdd = true
      },

      addLog () {
        this.$refs.logAddForm.validate()
        if (this.isEditedLogValid) {
          this.$store.dispatch('routineLogs/addLog', {
            editedItem: Object.assign({
              routine_id: this.editedItem.routine_id,
            }, this.editedLog),
          })
          this.closeLogAddDialog()
        }
      },

      closeLogAddDialog () {
        this.$refs.logAddForm.resetValidation()
        this.dialogLogAdd = false
      },
    }
  }
</script>
